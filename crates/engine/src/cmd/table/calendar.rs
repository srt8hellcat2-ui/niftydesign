//! Table > Calendar > Set Year: retype a calendar document for another year.
//!
//! A calendar month is a text frame whose whole text is an English month name and a four-digit
//! year, paired with a seven-column table on the same spread whose first row holds weekday names.
//! Only the heading's year digits and the day-number cells change; a cell holding anything else
//! is kept and reported.

use chrono::{Datelike, NaiveDate};
use designcraft_compose as compose;
use designcraft_doc::{Document, ItemId, StoryId, Table};
use designcraft_geom::Rect;
use serde_json::{Value, json};

use super::{bad, has_doc};
use crate::{Result, Session};

const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const WEEKDAYS: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
/// Most the frame of a table that gained rows may grow.
const MAX_GROWTH: f64 = 4000.0;
const MAX_NOTES_LISTED: usize = 5;
const MAX_CANDIDATES: usize = 5000;

struct Heading {
    story: StoryId,
    spread: usize,
    month: u32,
    year_at: usize,
    rect: Rect,
}

struct Frame {
    item: ItemId,
    story: StoryId,
    table: u64,
    spread: usize,
    rect: Rect,
}

enum Layout {
    /// Not a calendar: the first row holds no weekday names.
    No,
    Bad(String),
    /// Columns start on Monday (true) or Sunday (false).
    Ok {
        monday: bool,
    },
}

fn weekday(text: &str) -> Option<usize> {
    let t = text.trim().trim_end_matches('.');
    if t.is_empty() || t.chars().count() > 9 || !t.chars().all(|c| c.is_alphabetic()) {
        return None;
    }
    let head: String = t.chars().take(3).collect::<String>().to_lowercase();
    WEEKDAYS.iter().position(|w| *w == head)
}

fn layout(t: &Table) -> Layout {
    let days: Vec<Option<usize>> = (0..t.ncols()).map(|c| t.cell(0, c).and_then(|x| weekday(&x.text.text))).collect();
    if days.iter().all(Option::is_none) {
        return Layout::No;
    }
    if t.ncols() != 7 {
        return Layout::Bad(format!("it has {} columns, not 7", t.ncols()));
    }
    if !t.regions().is_empty() || t.cells.iter().any(|c| c.row_span != 1 || c.col_span != 1) {
        return Layout::Bad("it has merged cells".into());
    }
    let got: Vec<usize> = days.iter().flatten().copied().collect();
    if got == [0, 1, 2, 3, 4, 5, 6] {
        Layout::Ok { monday: false }
    } else if got == [1, 2, 3, 4, 5, 6, 0] {
        Layout::Ok { monday: true }
    } else {
        Layout::Bad("its first row is not Sun to Sat or Mon to Sun".into())
    }
}

/// `Month YYYY` as the whole text: (month 1..=12, byte offset of the year).
fn whole_heading(text: &str) -> Option<(u32, usize)> {
    let t = text.trim();
    let lead = text.len() - text.trim_start().len();
    let (m, y) = t.rsplit_once(char::is_whitespace)?;
    if y.len() != 4 || !y.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let m = m.trim_end();
    let month = MONTHS.iter().position(|n| n.eq_ignore_ascii_case(m))? as u32 + 1;
    Some((month, lead + t.len() - 4))
}

/// A month name and a year somewhere in a longer text.
fn contains_heading(text: &str) -> bool {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.windows(2).any(|w| MONTHS.iter().any(|n| n.eq_ignore_ascii_case(w[0])) && w[1].len() == 4 && w[1].bytes().all(|b| b.is_ascii_digit()))
}

fn days_in(year: i32, month: u32) -> Option<u32> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let next = if month == 12 { NaiveDate::from_ymd_opt(year.checked_add(1)?, 1, 1)? } else { NaiveDate::from_ymd_opt(year, month + 1, 1)? };
    u32::try_from((next - first).num_days()).ok()
}

fn is_day_number(text: &str) -> bool {
    let t = text.trim();
    !t.is_empty() && t.len() <= 2 && t.bytes().all(|b| b.is_ascii_digit()) && t.parse::<u32>().is_ok_and(|n| (1..=31).contains(&n))
}

fn page_name(d: &Document, spread: usize, rect: Rect) -> String {
    let page = d.spreads.get(spread).and_then(|s| s.page_at_x(rect.center().x)).unwrap_or(0);
    format!("page {}", d.page_name(d.first_page_of_spread(spread) + page))
}

fn gap(a: Rect, b: Rect) -> f64 {
    let dx = (a.x0 - b.x1).max(b.x0 - a.x1).max(0.0);
    let dy = (a.y0 - b.y1).max(b.y0 - a.y1).max(0.0);
    dx + dy
}

#[derive(Default)]
struct Report {
    months: usize,
    changed: Vec<Value>,
    warnings: Vec<String>,
    kept: usize,
}

/// Apply `year` to every calendar month of `d` (or only the table named by `only`).
fn apply(d: &mut Document, year: i32, only: Option<(Option<StoryId>, Option<u64>)>) -> Report {
    let mut rep = Report::default();
    let mut headings = Vec::new();
    let mut frames = Vec::new();
    for (si, spread) in d.spreads.iter().enumerate() {
        for top in &spread.items {
            top.walk(&mut |i| {
                let Some(tf) = i.text_frame() else { return };
                let Some(story) = d.story(tf.story) else { return };
                if frames.len() + headings.len() > MAX_CANDIDATES {
                    return;
                }
                if story.tables.is_empty() {
                    if let Some((month, year_at)) = whole_heading(&story.text) {
                        headings.push(Heading { story: tf.story, spread: si, month, year_at, rect: i.bounds() });
                    } else if contains_heading(&story.text) {
                        rep.warnings
                            .push(format!("{}: a month heading sits in a frame with other text and was not changed.", page_name(d, si, i.bounds())));
                    }
                } else {
                    for &table in story.tables.keys() {
                        frames.push(Frame { item: i.id, story: tf.story, table, spread: si, rect: i.bounds() });
                    }
                }
            });
        }
    }
    if headings.is_empty() {
        return rep;
    }
    // Nearest heading/table pairs on the same spread, one table per heading.
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for (h, head) in headings.iter().enumerate() {
        for (f, fr) in frames.iter().enumerate() {
            if fr.spread == head.spread && d.story(fr.story).and_then(|s| s.tables.get(&fr.table)).is_some_and(|t| !matches!(layout(t), Layout::No)) {
                pairs.push((gap(head.rect, fr.rect), h, f));
            }
        }
    }
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    let (mut head_done, mut frame_done) = (vec![false; headings.len()], vec![false; frames.len()]);
    let mut chosen = Vec::new();
    for (_, h, f) in pairs {
        if !head_done[h] && !frame_done[f] {
            head_done[h] = true;
            frame_done[f] = true;
            chosen.push((h, f));
        }
    }
    chosen.sort();
    for (h, head) in headings.iter().enumerate() {
        if !head_done[h] {
            let fits = only.is_none();
            if fits {
                rep.warnings.push(format!(
                    "{}: no calendar table with weekday names was found for the heading {}.",
                    page_name(d, head.spread, head.rect),
                    MONTHS[head.month as usize - 1]
                ));
            }
        }
    }
    for (h, f) in chosen {
        let (head, fr) = (&headings[h], &frames[f]);
        if let Some((s, t)) = only
            && (s.is_some_and(|s| s != fr.story) || t.is_some_and(|t| t != fr.table))
        {
            continue;
        }
        let place = page_name(d, head.spread, head.rect);
        let name = MONTHS[head.month as usize - 1];
        let Some(table) = d.story(fr.story).and_then(|s| s.tables.get(&fr.table)) else { continue };
        let monday = match layout(table) {
            Layout::Ok { monday } => monday,
            Layout::Bad(why) => {
                rep.warnings.push(format!("{place}: the {name} table was skipped because {why}."));
                continue;
            }
            Layout::No => continue,
        };
        let (Some(first), Some(days)) = (NaiveDate::from_ymd_opt(year, head.month, 1), days_in(year, head.month)) else {
            rep.warnings.push(format!("{place}: {name} {year} is outside the calendar range."));
            continue;
        };
        let sunday_based = first.weekday().num_days_from_sunday();
        let offset = if monday { (sunday_based + 6) % 7 } else { sunday_based } as usize;
        let weeks = (offset + days as usize).div_ceil(7);
        let need_rows = 1 + weeks;
        rep.months += 1;
        let mut changes = 0usize;
        // Heading year.
        if let Some(story) = d.story_mut(head.story)
            && story.text.get(head.year_at..head.year_at + 4).is_some_and(|y| y != year.to_string())
        {
            story.replace(head.year_at..head.year_at + 4, &year.to_string());
            changes += 1;
        }
        let mut notes: Vec<String> = Vec::new();
        let mut rows_before = 0;
        let mut rows_after = 0;
        let mut cell_changes = 0usize;
        if let Some(t) = d.story_mut(fr.story).and_then(|s| s.table_mut(fr.table)) {
            rows_before = t.nrows();
            if t.nrows() > need_rows {
                t.delete_rows(need_rows, t.nrows() - need_rows);
            } else if t.nrows() < need_rows {
                t.insert_rows(t.nrows(), need_rows - t.nrows());
            }
            rows_after = t.nrows();
            for r in 1..t.nrows() {
                for c in 0..7 {
                    let slot = (r - 1) * 7 + c;
                    let want = if slot >= offset && slot - offset < days as usize { (slot - offset + 1).to_string() } else { String::new() };
                    let Some(cell) = t.cell_mut(r, c) else { continue };
                    let have = cell.text.text.clone();
                    if have == want {
                        continue;
                    }
                    if !have.trim().is_empty() && !is_day_number(&have) {
                        notes.push(format!("\"{}\" (row {}, column {})", have.trim(), r + 1, c + 1));
                        continue;
                    }
                    let len = cell.text.len();
                    cell.text.replace(0..len, &want);
                    cell_changes += 1;
                }
            }
        }
        changes += cell_changes + rows_before.abs_diff(rows_after);
        let mut grew = 0.0;
        if rows_after > rows_before {
            match grow_frame(d, fr.item, fr.story) {
                Ok(g) => grew = g,
                Err(()) => rep
                    .warnings
                    .push(format!("{place}: the {name} table gained rows but its frame could not be made tall enough; the last rows may be hidden.")),
            }
        }
        if !notes.is_empty() {
            rep.kept += notes.len();
            let shown: Vec<_> = notes.iter().take(MAX_NOTES_LISTED).cloned().collect();
            rep.warnings.push(format!(
                "{place}: kept {} cell(s) in {name} {year} that are not day numbers: {}{}.",
                notes.len(),
                shown.join(", "),
                if notes.len() > MAX_NOTES_LISTED { ", …" } else { "" }
            ));
        }
        if changes > 0 {
            rep.changed.push(json!({"month": name, "year": year, "page": place, "story": fr.story.0, "table": fr.table,
                "rows": rows_after, "cellsChanged": cell_changes, "frameGrewBy": grew}));
        }
    }
    rep
}

/// Grow the table's frame downward until its story is no longer overset (at most `MAX_GROWTH`).
fn grow_frame(d: &mut Document, item: ItemId, story: StoryId) -> std::result::Result<f64, ()> {
    let overset = |d: &Document| compose::compose_story(d, story, &Default::default());
    if !overset(d).is_overset() {
        return Ok(0.0);
    }
    let inset = d.item(item).and_then(|i| i.text_frame()).map_or(0.0, |t| t.options.inset[0] + t.options.inset[2]);
    let local = d.item(item).and_then(|i| i.path.bounds()).ok_or(())?;
    let set = |d: &mut Document, h: f64| {
        if let Some(i) = d.item_mut(item) {
            i.path = designcraft_geom::shapes::rectangle(Rect::new(local.x0, local.y0, local.x1, local.y0 + h));
        }
    };
    set(d, local.height() + MAX_GROWTH);
    let tall = overset(d);
    let content = tall.frames.iter().map(|f| f.content_height).fold(0.0, f64::max);
    let needed = (content + inset + 0.5).max(local.height());
    if tall.is_overset() || !needed.is_finite() || needed > local.height() + MAX_GROWTH {
        set(d, local.height());
        return Err(());
    }
    set(d, needed);
    if overset(d).is_overset() {
        set(d, local.height());
        return Err(());
    }
    Ok(needed - local.height())
}

pub(super) fn set_calendar_year(s: &mut Session, p: &Value) -> Result<Value> {
    let year = p
        .get("year")
        .and_then(Value::as_i64)
        .filter(|y| (1900..=2200).contains(y))
        .ok_or_else(|| bad("table.setCalendarYear", "year must be a whole number from 1900 to 2200"))?;
    let only = {
        let story = p.get("story").and_then(Value::as_u64).map(StoryId);
        let table = p.get("table").and_then(Value::as_u64);
        (story.is_some() || table.is_some()).then_some((story, table))
    };
    let mut doc = (*s.doc()?.doc).clone();
    let rep = apply(&mut doc, year as i32, only);
    let message = if rep.months == 0 && rep.warnings.is_empty() {
        "No calendar month was found: a text frame holding only a month name and year (like \"May 2026\") next to a table whose first row holds weekday names. Nothing was changed.".to_string()
    } else if rep.changed.is_empty() {
        format!("{} month(s) already match {year}; nothing was changed.", rep.months)
    } else {
        format!("Set {} month(s) to {year}; {} day cell(s) with other text were kept.", rep.changed.len(), rep.kept)
    };
    if !rep.changed.is_empty() {
        s.edit(|d, _| {
            *d = doc;
            Ok(())
        })?;
    }
    let mut out = json!({"months": rep.months, "changed": rep.changed, "warnings": rep.warnings, "kept": rep.kept, "message": message});
    if rep.months == 0 && rep.warnings.is_empty() {
        out["changed"] = json!([]);
    }
    Ok(out)
}

/// The calendar year of a Unix time in seconds (UTC), kept within the command's range; the dialog's default.
pub fn year_of_unix_secs(secs: i64) -> i32 {
    chrono::DateTime::from_timestamp(secs, 0).map_or(2026, |t| t.year()).clamp(1900, 2200)
}

pub(super) fn enabled(s: &Session) -> std::result::Result<(), String> {
    has_doc(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use designcraft_doc::StoryId;

    /// Day of the week (0 = Sunday) of year-month-1, by Sakamoto's formula, independent of chrono.
    fn weekday_of_first(y: i32, m: i32) -> usize {
        const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = if m < 3 { y - 1 } else { y };
        ((y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + 1) % 7) as usize
    }
    fn days(y: i32, m: i32) -> usize {
        match m {
            2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
    /// The body rows a correct calendar shows.
    fn reference(y: i32, m: i32, monday: bool) -> Vec<Vec<String>> {
        let offset = (weekday_of_first(y, m) + if monday { 6 } else { 0 }) % 7;
        let weeks = (offset + days(y, m)).div_ceil(7);
        (0..weeks)
            .map(|w| {
                (0..7).map(|c| (w * 7 + c).checked_sub(offset).filter(|d| *d < days(y, m)).map_or(String::new(), |d| (d + 1).to_string())).collect()
            })
            .collect()
    }
    const NAMES: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const HEAD_SUN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const HEAD_MON: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    struct Cal {
        heading: u64,
        story: u64,
        table: u64,
    }
    /// A month heading frame and a table frame (header row + `body` rows) at vertical offset `y`.
    fn add(s: &mut Session, y: f64, month: i32, year: i32, monday: bool, body: &[Vec<String>], heading: &str) -> Cal {
        let h = s.execute("frame.create", &json!({"rect": [36, y, 336, y + 24.0], "content": "text", "text": heading})).unwrap();
        let f = s.execute("frame.create", &json!({"rect": [36, y + 30.0, 540, y + 330.0], "content": "text", "text": ""})).unwrap();
        s.execute("text.select", &json!({"story": f["story"], "anchor": 0, "focus": 0})).unwrap();
        let t = s.execute("table.insert", &json!({"rows": body.len() + 1, "cols": 7})).unwrap();
        let (story, table) = (f["story"].as_u64().unwrap(), t["table"].as_u64().unwrap());
        let head = if monday { HEAD_MON } else { HEAD_SUN };
        for (c, name) in head.iter().enumerate() {
            s.execute("table.setCell", &json!({"story": story, "table": table, "rows": [0, 0], "cols": [c, c], "text": name})).unwrap();
        }
        for (r, row) in body.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                s.execute("table.setCell", &json!({"story": story, "table": table, "rows": [r + 1, r + 1], "cols": [c, c], "text": v})).unwrap();
            }
        }
        let _ = (month, year);
        Cal { heading: h["story"].as_u64().unwrap(), story, table }
    }
    fn session() -> Session {
        let mut s = Session::new();
        s.execute("file.new", &json!({})).unwrap();
        s
    }
    fn body(s: &Session, c: &Cal) -> Vec<Vec<String>> {
        let t = s.doc().unwrap().doc.story(StoryId(c.story)).unwrap().tables.get(&c.table).unwrap();
        (1..t.nrows()).map(|r| (0..t.ncols()).map(|k| t.cell(r, k).unwrap().text.text.clone()).collect()).collect()
    }
    fn heading(s: &Session, c: &Cal) -> String {
        s.doc().unwrap().doc.story(StoryId(c.heading)).unwrap().text.clone()
    }
    fn snapshot(s: &Session) -> Value {
        serde_json::to_value(&*s.doc().unwrap().doc).unwrap()
    }
    fn run(s: &mut Session, year: i32) -> Value {
        s.execute("table.setCalendarYear", &json!({"year": year})).unwrap()
    }

    #[test]
    fn the_reference_matches_the_known_facts() {
        // Sunday-first: (year, month, first-day weekday, body rows + header).
        for (y, m, wd, rows) in [(2027, 5, 6, 7), (2027, 6, 2, 6), (2027, 7, 4, 6), (2027, 8, 0, 6), (2028, 7, 6, 7)] {
            assert_eq!(weekday_of_first(y, m), wd);
            assert_eq!(reference(y, m, false).len() + 1, rows, "{y}-{m}");
        }
    }

    #[test]
    fn sunday_and_monday_weeks_and_row_counts_follow_the_new_year() {
        for (year, month, monday) in [(2027, 5, false), (2027, 6, false), (2027, 7, false), (2027, 8, true), (2028, 7, true), (2028, 7, false)] {
            let mut s = session();
            let old = reference(2026, month, monday);
            let c = add(&mut s, 20.0, month, 2026, monday, &old, &format!("{} 2026", NAMES[month as usize - 1]));
            let r = run(&mut s, year);
            assert_eq!(r["months"], 1, "{r}");
            assert_eq!(body(&s, &c), reference(year, month, monday), "{year}-{month} monday={monday}");
            assert_eq!(heading(&s, &c), format!("{} {year}", NAMES[month as usize - 1]));
        }
    }

    #[test]
    fn leap_years_are_right_including_1900_and_2100() {
        for year in [2024, 2028, 1900, 2100, 2023] {
            let mut s = session();
            let start = if year == 2024 { 2023 } else { 2024 };
            let c = add(&mut s, 20.0, 2, start, false, &reference(start, 2, false), &format!("February {start}"));
            run(&mut s, year);
            let got = body(&s, &c);
            assert_eq!(got, reference(year, 2, false), "{year}");
            let n = got.iter().flatten().filter(|v| !v.is_empty()).count();
            assert_eq!(n, if matches!(year, 2024 | 2028) { 29 } else { 28 }, "{year}");
        }
    }

    #[test]
    fn a_seven_row_month_becomes_six_rows_and_back_with_a_grown_frame() {
        let mut s = session();
        // May 2027 has 7 rows; April 2027... use January 2026 (Thursday, 5 weeks => 6 rows) <-> May 2027.
        let c = add(&mut s, 20.0, 5, 2027, false, &reference(2027, 5, false), "May 2027");
        assert_eq!(body(&s, &c).len(), 6);
        let r = run(&mut s, 2028); // May 2028 starts Monday: 1 + 31 => 5 weeks
        assert_eq!(body(&s, &c), reference(2028, 5, false));
        assert_eq!(body(&s, &c).len(), 5, "{r}");
        // Back to a 6-week month: fit the frame exactly to the 5-week table first.
        let sid = StoryId(c.story);
        let item = s.doc().unwrap().doc.stories[&sid].frames[0];
        s.edit(|d, _| {
            let tall = compose::compose_story(d, sid, &Default::default());
            let h = tall.frames.iter().map(|f| f.content_height).fold(0.0, f64::max);
            let b = d.item(item).unwrap().path.bounds().unwrap();
            d.item_mut(item).unwrap().path = designcraft_geom::shapes::rectangle(Rect::new(b.x0, b.y0, b.x1, b.y0 + h + 0.6));
            Ok(())
        })
        .unwrap();
        let before = s.doc().unwrap().doc.item(item).unwrap().bounds().height();
        let r = run(&mut s, 2027);
        assert_eq!(body(&s, &c), reference(2027, 5, false));
        let after = s.doc().unwrap().doc.item(item).unwrap().bounds().height();
        assert!(after > before, "the frame grew: {before} -> {after}; {r}");
        let composed = compose::compose_story(&s.doc().unwrap().doc, sid, &Default::default());
        assert!(!composed.is_overset());
        assert!(r["changed"][0]["frameGrewBy"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn the_same_year_twice_changes_nothing() {
        let mut s = session();
        let _ = add(&mut s, 20.0, 7, 2027, false, &reference(2027, 7, false), "July 2027");
        let steps = s.doc().unwrap().history.undo.len();
        let before = snapshot(&s);
        let r = run(&mut s, 2027);
        assert!(r["changed"].as_array().unwrap().is_empty(), "{r}");
        assert_eq!(snapshot(&s), before);
        assert!(s.doc().unwrap().history.undo.len() <= steps + 1);
        assert_eq!(r["months"], 1);
    }

    #[test]
    fn one_undo_restores_everything_exactly() {
        let mut s = session();
        let c = add(&mut s, 20.0, 5, 2026, false, &reference(2026, 5, false), "May 2026");
        let _ = add(&mut s, 400.0, 6, 2026, true, &reference(2026, 6, true), "June 2026");
        let before = snapshot(&s);
        let steps = s.doc().unwrap().history.undo.len();
        let r = run(&mut s, 2028);
        assert_eq!(r["months"], 2, "{r}");
        assert_eq!(s.doc().unwrap().history.undo.len(), steps + 1);
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(snapshot(&s), before);
        assert_eq!(heading(&s, &c), "May 2026");
    }

    #[test]
    fn a_typed_note_is_kept_and_reported_and_formatting_stays() {
        let mut s = session();
        let mut old = reference(2026, 7, false);
        old[2][3] = "Dentist".into();
        let c = add(&mut s, 20.0, 7, 2026, false, &old, "July 2026");
        // The first cell of the second week (row 2, column 0) is bold; it holds 5 now and 4 in 2027.
        let sid = StoryId(c.story);
        s.edit(|d, _| {
            let t = d.story_mut(sid).unwrap().table_mut(c.table).unwrap();
            let cell = t.cell_mut(2, 0).unwrap();
            cell.text.format_chars(0..1, |f| f.over.font_style = Some("Bold".into()));
            Ok(())
        })
        .unwrap();
        let r = run(&mut s, 2027);
        let got = body(&s, &c);
        let mut want = reference(2027, 7, false);
        want[2][3] = "Dentist".into();
        assert_eq!(got, want);
        assert_eq!(r["kept"], 1);
        assert!(r["warnings"][0].as_str().unwrap().contains("Dentist"), "{r}");
        let t = s.doc().unwrap().doc.story(sid).unwrap().tables.get(&c.table).unwrap();
        let cell = t.cell(2, 0).unwrap();
        assert_eq!(cell.text.text, "4");
        assert_eq!(cell.text.char_format_at(0).over.font_style.as_deref(), Some("Bold"), "retyping keeps the cell's character format");
    }

    #[test]
    fn a_heading_in_a_frame_with_other_text_is_skipped() {
        let mut s = session();
        let old = reference(2026, 5, false);
        let c = add(&mut s, 20.0, 5, 2026, false, &old, "May 2026 and some notes");
        let before = snapshot(&s);
        let r = run(&mut s, 2027);
        assert_eq!(snapshot(&s), before);
        assert_eq!(r["months"], 0);
        assert!(r["warnings"][0].as_str().unwrap().contains("other text"), "{r}");
        assert_eq!(heading(&s, &c), "May 2026 and some notes");
    }

    #[test]
    fn a_document_without_a_calendar_is_left_alone_and_bad_tables_are_skipped() {
        let mut s = session();
        s.execute("frame.create", &json!({"rect": [0, 0, 200, 50], "content": "text", "text": "Hello"})).unwrap();
        let before = snapshot(&s);
        let r = run(&mut s, 2027);
        assert_eq!(snapshot(&s), before);
        assert!(r["message"].as_str().unwrap().contains("Nothing was changed"));
        assert!(s.execute("table.setCalendarYear", &json!({"year": 1899})).is_err());
        assert!(s.execute("table.setCalendarYear", &json!({"year": 2201})).is_err());
        // A weekday header with 5 columns.
        let h = s.execute("frame.create", &json!({"rect": [36, 100, 336, 124], "content": "text", "text": "May 2026"})).unwrap();
        let f = s.execute("frame.create", &json!({"rect": [36, 130, 540, 400], "content": "text", "text": ""})).unwrap();
        s.execute("text.select", &json!({"story": f["story"], "anchor": 0, "focus": 0})).unwrap();
        let t = s.execute("table.insert", &json!({"rows": 3, "cols": 5})).unwrap();
        for (c, name) in HEAD_SUN.iter().enumerate().take(5) {
            s.execute("table.setCell", &json!({"story": f["story"], "table": t["table"], "rows": [0, 0], "cols": [c, c], "text": name})).unwrap();
        }
        let before = snapshot(&s);
        let r = run(&mut s, 2027);
        assert_eq!(snapshot(&s), before, "{r}");
        assert!(r["warnings"][0].as_str().unwrap().contains("5 columns"), "{r}");
        let _ = h;
    }
}

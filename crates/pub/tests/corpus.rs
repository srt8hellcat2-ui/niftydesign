// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Genuine Publisher files are not redistributable, so they are never committed: point
//! `PUB_CORPUS` at local folders of `.pub` files (`:`-separated). A folder may hold Publisher's own
//! layout exports in `truth/<name>.json` (FuncularLabs/pubsmith fixtures), which the report
//! compares against (the save/reopen/PDF pass is `crates/engine/tests/pub_corpus.rs`). Run: `PUB_CORPUS=dir1:dir2 cargo test -p designcraft-pub --test corpus -- --ignored --nocapture`.

use std::path::{Path, PathBuf};

use designcraft_doc::{Content, Document, Item};

fn corpus() -> Vec<PathBuf> {
    let dirs = std::env::var("PUB_CORPUS").expect("set PUB_CORPUS to folders of .pub files");
    let mut v: Vec<PathBuf> = dirs
        .split(':')
        .filter(|d| !d.is_empty())
        .flat_map(|d| std::fs::read_dir(d).into_iter().flatten().flatten().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("pub")))
        .collect();
    v.sort();
    v
}

/// Leaf items: group members instead of their groups (Publisher's layout export lists members).
fn leaves<'a>(it: &'a Item, out: &mut Vec<&'a Item>) {
    match &it.content {
        Content::Group { items } => items.iter().for_each(|c| leaves(c, out)),
        _ => out.push(it),
    }
}

fn items(d: &Document) -> Vec<&Item> {
    let mut out = Vec::new();
    for sp in &d.spreads {
        for it in &sp.items {
            leaves(it, &mut out);
        }
    }
    out
}

/// Items drawn on document pages: their own and their parents' (Publisher exports master items
/// on every page).
fn shown_items(d: &Document) -> Vec<&Item> {
    let mut out = Vec::new();
    for sp in d.spreads.iter().chain(&d.parents) {
        for it in &sp.items {
            leaves(it, &mut out);
        }
    }
    out
}

fn story_text(d: &Document, it: &Item) -> Option<String> {
    let Content::Text(t) = &it.content else { return None };
    d.stories.get(&t.story).map(|s| s.text.clone())
}

#[derive(Default)]
struct Score {
    elements: usize,
    found: usize,
    geometry_ok: usize,
    text_ok: usize,
    text_total: usize,
    style_checked: usize,
    style_ok: usize,
}

fn swatch_hex(d: &Document, name: &str) -> Option<String> {
    let sw = d.swatch(name)?;
    match &sw.value {
        designcraft_color::SwatchValue::Color { color, .. } => {
            let [r, g, b, _] = color.to_rgba8(1.0);
            Some(format!("#{r:02X}{g:02X}{b:02X}"))
        }
        designcraft_color::SwatchValue::Paper { .. } => Some("#FFFFFF".into()),
        _ => None,
    }
}

/// Compare with Publisher's layout export: each exported element needs an item of the same kind
/// whose (unrotated) frame matches within 1 pt; text elements also need equal text.
fn score(d: &Document, truth: &serde_json::Value, s: &mut Score, notes: &mut Vec<String>) {
    let Some(pages) = truth["pages"].as_array() else { return };
    if pages.len() != d.page_count() {
        notes.push(format!("pages {} vs truth {}", d.page_count(), pages.len()));
    }
    let all = shown_items(d);
    for (pi, p) in pages.iter().enumerate() {
        if let Some(page) = d.page(pi)
            && ((page.width - p["width"].as_f64().unwrap_or(0.0)).abs() > 0.5 || (page.height - p["height"].as_f64().unwrap_or(0.0)).abs() > 0.5)
        {
            notes.push(format!("page {pi} size {}x{}", page.width, page.height));
        }
        let x0 = d.page(pi).map_or(0.0, |pg| pg.x);
        for e in p["elements"].as_array().into_iter().flatten() {
            s.elements += 1;
            let b = &e["bounds"];
            let (x, y, w, h) = (
                b["x"].as_f64().unwrap_or(0.0) + x0,
                b["y"].as_f64().unwrap_or(0.0),
                b["width"].as_f64().unwrap_or(0.0),
                b["height"].as_f64().unwrap_or(0.0),
            );
            let want_text = e["type"] == "text";
            let want_image = e["type"] == "image";
            // Publisher's export turns some objects into pictures; any item over them counts.
            let cands: Vec<&&Item> = all
                .iter()
                .filter(|it| {
                    let r = it.inner_bounds();
                    // A filled shape holding text is one item: it answers for both the shape and its text.
                    let ok_kind = want_image || !want_text || it.is_text_frame();
                    ok_kind && r.x0 < x + w && r.x1 > x && r.y0 < y + h && r.y1 > y
                })
                .collect();
            let Some(best) = cands.iter().min_by(|a, b| {
                let da = (a.inner_bounds().x0 - x).abs() + (a.inner_bounds().y0 - y).abs();
                let db = (b.inner_bounds().x0 - x).abs() + (b.inner_bounds().y0 - y).abs();
                da.total_cmp(&db)
            }) else {
                notes.push(format!("missing {}", e["type"]));
                continue;
            };
            s.found += 1;
            if !want_image {
                let r = best.inner_bounds();
                let err = [(r.x0 - x), (r.y0 - y), (r.width() - w), (r.height() - h)].iter().fold(0f64, |m, v| m.max(v.abs()));
                if err <= 1.0 { s.geometry_ok += 1 } else { notes.push(format!("{} off by {err:.1} pt", e["type"])) }
            }
            // Styling: shape fill colour and line weight; first text run's font, size, weight, colour.
            if e["type"] == "shape" {
                let mut bad = Vec::new();
                if let Some(f) = e["fill"].as_str().filter(|f| f.len() == 7) {
                    if swatch_hex(d, &best.fill.swatch).as_deref() != Some(&f.to_uppercase()) {
                        bad.push(format!("fill {f} vs {}", best.fill.swatch));
                    }
                } else if e["fill"].is_null() && !best.fill.is_none() {
                    bad.push("fill should be none".into());
                }
                let want_w = e["stroke"]["width"].as_f64().unwrap_or(0.0);
                let got_w = if best.stroke.is_none() { 0.0 } else { best.stroke.weight };
                if (want_w - got_w).abs() > 0.05 {
                    bad.push(format!("stroke {want_w} vs {got_w}"));
                }
                s.style_checked += 1;
                if bad.is_empty() { s.style_ok += 1 } else { notes.push(bad.join(", ")) }
            }
            if want_text && let Some(run) = e["paragraphs"][0]["runs"][0].as_object() {
                let st = &run["style"];
                if let Content::Text(t) = &best.content
                    && let Some(story) = d.stories.get(&t.story)
                {
                    let cf = story.char_format_at(0);
                    let a = &cf.over;
                    let attrs = serde_json::to_value(a).unwrap_or_default();
                    let mut bad = Vec::new();
                    if let Some(fam) = st["family"].as_str()
                        && attrs["fontFamily"].as_str() != Some(fam)
                    {
                        bad.push(format!("font {fam} vs {}", attrs["fontFamily"]));
                    }
                    if let Some(sz) = st["size"].as_f64()
                        && (attrs["size"].as_f64().unwrap_or(12.0) - sz).abs() > 0.05
                    {
                        bad.push(format!("size {sz} vs {}", attrs["size"]));
                    }
                    let face = attrs["fontStyle"].as_str().unwrap_or("Regular");
                    if st["bold"].as_bool().unwrap_or(false) != face.contains("Bold")
                        || st["italic"].as_bool().unwrap_or(false) != face.contains("Italic")
                    {
                        bad.push(format!("face {face}"));
                    }
                    if let Some(c) = st["color"].as_str()
                        && swatch_hex(d, attrs["fill"].as_str().unwrap_or("[Black]")).as_deref() != Some(&c.to_uppercase())
                    {
                        bad.push(format!("colour {c} vs {}", attrs["fill"]));
                    }
                    s.style_checked += 1;
                    if bad.is_empty() { s.style_ok += 1 } else { notes.push(bad.join(", ")) }
                }
            }
            if want_text {
                s.text_total += 1;
                let want: Vec<String> = e["paragraphs"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|p| p["runs"].as_array().into_iter().flatten().filter_map(|r| r["text"].as_str()).collect::<String>())
                    .collect();
                let got = story_text(d, best).unwrap_or_default();
                let norm = |s: &str| s.replace(['\u{2028}', '\u{b}', '\r'], "\n").trim_end().to_string();
                if norm(&got) == norm(&want.join("\n")) { s.text_ok += 1 } else { notes.push(format!("text {got:?} vs {:?}", want.join("\n"))) }
            }
        }
    }
}

#[test]
#[ignore = "needs PUB_CORPUS: genuine Publisher files are not redistributable"]
fn corpus_import_report_nfty_8c51ad1b() {
    let files = corpus();
    assert!(!files.is_empty(), "no .pub files in PUB_CORPUS");
    let (mut ok, mut failed) = (0, 0);
    let mut total = Score::default();
    let mut note_counts: std::collections::BTreeMap<String, usize> = Default::default();
    let mut warning_counts: std::collections::BTreeMap<String, usize> = Default::default();
    for f in &files {
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        let bytes = std::fs::read(f).unwrap();
        match designcraft_pub::import_pub(&bytes) {
            Err(e) => {
                failed += 1;
                println!("FAIL  {name}: {e}");
            }
            Ok(imp) => {
                ok += 1;
                let d = &imp.document;
                let its = items(d);
                let texts = its.iter().filter(|i| i.is_text_frame()).count();
                let pics = its.iter().filter(|i| matches!(i.content, Content::Graphic(_))).count();
                let mut line = format!(
                    "OK    {name}: pages {} items {} (text {texts}, pictures {pics}) warnings {}",
                    d.page_count(),
                    its.len(),
                    imp.warnings.len()
                );
                for note in &imp.notes {
                    *note_counts.entry(note.clone()).or_default() += 1;
                }
                for w in &imp.warnings {
                    *warning_counts.entry(w.clone()).or_default() += 1;
                }
                let truth = f.parent().unwrap_or(Path::new(".")).join("truth").join(name.replace(".pub", ".json"));
                if let Ok(t) = std::fs::read_to_string(&truth) {
                    let mut s = Score::default();
                    let mut notes = Vec::new();
                    score(d, &serde_json::from_str(&t).unwrap(), &mut s, &mut notes);
                    line += &format!(
                        " | truth: found {}/{} geometry {} text {}/{} style {}/{}",
                        s.found, s.elements, s.geometry_ok, s.text_ok, s.text_total, s.style_ok, s.style_checked
                    );
                    if !notes.is_empty() {
                        line += &format!(" [{}]", notes.join("; "));
                    }
                    total.elements += s.elements;
                    total.found += s.found;
                    total.geometry_ok += s.geometry_ok;
                    total.text_ok += s.text_ok;
                    total.text_total += s.text_total;
                    total.style_ok += s.style_ok;
                    total.style_checked += s.style_checked;
                }
                if std::env::var_os("PUB_VERBOSE").is_some() {
                    for w in &imp.warnings {
                        line += &format!("\n      - {w}");
                    }
                }
                println!("{line}");
            }
        }
    }
    println!(
        "SUMMARY files {} imported {ok} failed {failed}; truth elements {} found {} geometry-within-1pt {} text exact {}/{} styling {}/{}",
        files.len(),
        total.elements,
        total.found,
        total.geometry_ok,
        total.text_ok,
        total.text_total,
        total.style_ok,
        total.style_checked
    );
    for (note, n) in note_counts {
        println!("NOTE  {n:4}× {note}");
    }
    for (w, n) in warning_counts {
        println!("WARN  {n:4}× {w}");
    }
}

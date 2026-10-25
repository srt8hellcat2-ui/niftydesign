// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Genuine Publisher files are not redistributable, so they are never committed: point
//! `PUB_CORPUS` at local folders of `.pub` files (`:`-separated). A folder may hold Publisher's own
//! layout exports in `truth/<name>.json` (FuncularLabs/pubsmith fixtures), which the report
//! compares against (the save/reopen/PDF pass is `crates/engine/tests/pub_corpus.rs`). Run: `PUB_CORPUS=dir1:dir2 cargo test -p designcraft-pub --test corpus -- --ignored --nocapture`.

use std::path::{Path, PathBuf};

use designcraft_doc::{Content, Document, Item, ItemId, SpreadRef};
use designcraft_geom::{Affine, Rect};

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
    geometry_total: usize,
    picture_total: usize,
    picture_ok: usize,
    picture_unmatched: usize,
    text_ok: usize,
    text_total: usize,
    style_checked: usize,
    style_ok: usize,
}

#[derive(Clone, Copy)]
struct PictureBounds {
    id: ItemId,
    bounds: Rect,
}

/// Painted placed-frame bounds: transformed frame outline plus its visible stroke.
/// The source image's uncropped rectangle is deliberately not used.
fn picture_leaves(it: &Item, outer: Affine, depth: usize, out: &mut Vec<PictureBounds>) {
    if depth >= 64 || it.hidden {
        return;
    }
    if let Content::Group { items } = &it.content {
        for child in items {
            picture_leaves(child, outer * it.xf, depth + 1, out);
        }
    } else if matches!(it.content, Content::Graphic(_)) {
        let mut placed = it.clone();
        placed.xf = outer * it.xf;
        let bounds = placed.visible_bounds();
        if [bounds.x0, bounds.y0, bounds.x1, bounds.y1].iter().all(|v| v.is_finite()) {
            out.push(PictureBounds { id: it.id, bounds });
        }
    }
}

/// Scope candidates to the shown page, including only its applied, non-overridden parents.
fn page_pictures(d: &Document, page_index: usize) -> Vec<PictureBounds> {
    let mut out = Vec::new();
    let Some((si, pi)) = d.page_loc(page_index) else { return out };
    let Some(spread) = d.spreads.get(si) else { return out };
    let Some(page) = spread.pages.get(pi) else { return out };
    for it in &spread.items {
        if spread.pages.len() == 1 || spread.page_at_x(it.bounds().center().x) == Some(pi) {
            picture_leaves(it, Affine::IDENTITY, 0, &mut out);
        }
    }
    if page.show_parent_items {
        for (parent_index, parent_page) in d.shown_parent_pages(SpreadRef::Doc(si), pi) {
            let Some(parent) = d.parents.get(parent_index) else { continue };
            let Some(pp) = parent.pages.get(parent_page) else { continue };
            for it in &parent.items {
                if !page.overridden.contains(&it.id) && (parent.pages.len() == 1 || parent.page_at_x(it.bounds().center().x) == Some(parent_page)) {
                    picture_leaves(it, Affine::translate((page.x - pp.x, 0.0)), 0, &mut out);
                }
            }
        }
    }
    out
}

fn bounds_errors(ours: Rect, expected: Rect) -> [f64; 4] {
    [ours.x0 - expected.x0, ours.y0 - expected.y0, ours.width() - expected.width(), ours.height() - expected.height()]
}

/// Closest unused reference/picture pair by L1 distance over x/y/width/height.
/// Greedily consume the globally nearest pair, so reference order cannot let a distant
/// rasterized non-picture take a picture from a nearer reference. No overlap requirement.
fn match_pictures(pictures: &[PictureBounds], expected: &[Rect]) -> Vec<Option<usize>> {
    let mut matches = vec![None; expected.len()];
    let mut used = std::collections::BTreeSet::new();
    for _ in 0..expected.len().min(pictures.len()) {
        let mut best: Option<(f64, usize, usize, ItemId)> = None;
        for (ei, rect) in expected.iter().enumerate() {
            if matches.get(ei).is_some_and(Option::is_some) {
                continue;
            }
            for (pi, picture) in pictures.iter().enumerate().filter(|(_, p)| !used.contains(&p.id)) {
                let distance = bounds_errors(picture.bounds, *rect).iter().map(|e| e.abs()).sum::<f64>();
                let closer = best.is_none_or(|(d, e, _, id)| distance.total_cmp(&d).then(ei.cmp(&e)).then(picture.id.cmp(&id)).is_lt());
                if closer {
                    best = Some((distance, ei, pi, picture.id));
                }
            }
        }
        let Some((_, ei, pi, id)) = best else { break };
        if let Some(slot) = matches.get_mut(ei) {
            *slot = Some(pi);
            used.insert(id);
        }
    }
    matches
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

/// Preserve the historical shape/text checks and overlap coverage; separately compare
/// image-labelled exports with one-to-one picture candidates in painted placed-frame bounds.
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
        let pictures = page_pictures(d, pi);
        let expected_pictures: Vec<Rect> = p["elements"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|e| e["type"] == "image")
            .map(|e| {
                let b = &e["bounds"];
                let x = b["x"].as_f64().unwrap_or(0.0) + x0;
                let y = b["y"].as_f64().unwrap_or(0.0);
                Rect::new(x, y, x + b["width"].as_f64().unwrap_or(0.0), y + b["height"].as_f64().unwrap_or(0.0))
            })
            .collect();
        // A parent picture shown on another page is a separate displayed instance.
        let picture_matches = match_pictures(&pictures, &expected_pictures);
        let mut picture_index = 0;
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
            if want_image {
                s.picture_total += 1;
                let expected = Rect::new(x, y, x + w, y + h);
                let picture = picture_matches.get(picture_index).and_then(|index| index.and_then(|i| pictures.get(i)));
                picture_index += 1;
                if let Some(picture) = picture {
                    let errors = bounds_errors(picture.bounds, expected);
                    let max_error = errors.iter().fold(0.0_f64, |m, e| m.max(e.abs()));
                    if max_error <= 1.0 {
                        s.picture_ok += 1;
                    }
                    notes.push(format!(
                        "picture page {} item {} expected ({x:.6},{y:.6},{w:.6},{h:.6}) ours ({:.6},{:.6},{:.6},{:.6}) errors ({:.6},{:.6},{:.6},{:.6}) pt",
                        pi + 1, picture.id.0, picture.bounds.x0, picture.bounds.y0, picture.bounds.width(), picture.bounds.height(),
                        errors[0], errors[1], errors[2], errors[3]
                    ));
                } else {
                    s.picture_unmatched += 1;
                    notes.push(format!("picture page {} unmatched: no unused imported picture", pi + 1));
                }
                // Preserve historical spatial-coverage totals, not picture identity or position.
                // An image-labelled export of WordArt, a table or a shape still has coverage.
                if all.iter().any(|it| {
                    let r = it.inner_bounds();
                    r.x0 < x + w && r.x1 > x && r.y0 < y + h && r.y1 > y
                }) {
                    s.found += 1;
                } else {
                    notes.push(format!("missing {}", e["type"]));
                }
                continue;
            }
            s.geometry_total += 1;
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
                    total.geometry_total += s.geometry_total;
                    total.picture_total += s.picture_total;
                    total.picture_ok += s.picture_ok;
                    total.picture_unmatched += s.picture_unmatched;
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
    println!("POSITIONS shapes/text: {} of {} scored objects within 1 pt", total.geometry_ok, total.geometry_total);
    println!(
        "POSITIONS pictures: {} of {} pictures within 1 pt; {} unmatched (reference image elements; imported pictures only; informational)",
        total.picture_ok, total.picture_total, total.picture_unmatched
    );
    println!("COVERAGE legacy found totals above measure overlap, not one-to-one picture matches.");
    for (note, n) in note_counts {
        println!("NOTE  {n:4}× {note}");
    }
    for (w, n) in warning_counts {
        println!("WARN  {n:4}× {w}");
    }
}

#[cfg(test)]
mod picture_tests {
    use super::*;
    use designcraft_doc::{AssetId, Fill, Graphic, LayerId, Shape, Stroke, build::NewDocument};
    use designcraft_geom::shapes;
    use std::sync::Arc;

    fn picture(id: u64, rect: Rect) -> Item {
        let mut it = Item::new(ItemId(id), LayerId(1), Shape::Rectangle, shapes::rectangle(rect));
        it.content = Content::Graphic(Graphic {
            asset: AssetId(1),
            size: (10.0, 20.0),
            xf: Affine::IDENTITY,
            clip: None,
            auto_fit: Default::default(),
            fit_align: 4,
            crop: [0.0; 4],
            wrap: Default::default(),
            fill: Fill::none(),
        });
        it.stroke = Stroke::none();
        it
    }

    #[test]
    fn nearest_picture_uses_size_as_well_as_origin_and_never_reuses_an_item() {
        let small = Rect::new(0.0, 0.0, 10.0, 10.0);
        let large = Rect::new(0.0, 0.0, 20.0, 20.0);
        let pictures = [PictureBounds { id: ItemId(1), bounds: small }, PictureBounds { id: ItemId(2), bounds: large }];
        assert_eq!(match_pictures(&pictures, &[large, small, small]), vec![Some(1), Some(0), None]);
    }

    #[test]
    fn nearer_reference_wins_before_a_distant_rasterized_shape() {
        let rect = Rect::new(100.0, 100.0, 110.0, 110.0);
        let pictures = [PictureBounds { id: ItemId(1), bounds: rect }];
        assert_eq!(match_pictures(&pictures, &[Rect::new(0.0, 0.0, 10.0, 10.0), rect]), vec![None, Some(0)]);
        // A genuine displaced picture still gets measured, without an overlap filter.
        assert_eq!(match_pictures(&pictures, &[Rect::new(0.0, 0.0, 10.0, 10.0)]), vec![Some(0)]);
    }

    #[test]
    fn shapes_and_text_are_not_picture_candidates() {
        let mut d = Document::new(&NewDocument { facing_pages: false, ..Default::default() });
        let layer = d.layers[0].id;
        let rect = Rect::new(0.0, 0.0, 10.0, 20.0);
        d.add_text_frame(SpreadRef::Doc(0), rect, layer, "text", Default::default()).unwrap();
        let mut shape = picture(100, rect);
        shape.content = Content::Unassigned;
        d.insert_item(SpreadRef::Doc(0), shape, None).unwrap();
        let expected =
            serde_json::json!({"pages":[{"width":612,"height":792,"elements":[{"type":"image","bounds":{"x":0,"y":0,"width":10,"height":20}}]}]});
        let mut s = Score::default();
        score(&d, &expected, &mut s, &mut Vec::new());
        assert_eq!((s.found, s.geometry_total, s.picture_total, s.picture_ok, s.picture_unmatched), (1, 0, 1, 0, 1));
    }

    #[test]
    fn grouped_rotation_flip_and_border_are_in_placed_bounds() {
        let mut child = picture(1, Rect::new(0.0, 0.0, 10.0, 20.0));
        child.xf = Affine::scale_non_uniform(-1.0, 1.0);
        child.stroke = Stroke { weight: 2.0, ..Default::default() };
        let mut group = Item::new(ItemId(2), LayerId(1), Shape::Group, Default::default());
        group.xf = Affine::translate((100.0, 50.0)) * Affine::rotate(std::f64::consts::FRAC_PI_2);
        group.content = Content::Group { items: vec![Arc::new(child)] };
        let mut candidates = Vec::new();
        picture_leaves(&group, Affine::IDENTITY, 0, &mut candidates);
        assert_eq!(candidates.len(), 1);
        // Mirrored child spans x=-10..0,y=0..20; turn gives x=-20..0,y=-10..0.
        // Translation and a 1 pt border give x=79..101,y=39..51.
        let errors = bounds_errors(candidates[0].bounds, Rect::new(79.0, 39.0, 101.0, 51.0));
        assert!(errors.iter().all(|e| e.abs() < 1e-9));
    }

    #[test]
    fn cropped_image_rectangle_does_not_replace_the_frame() {
        let rect = Rect::new(20.0, 30.0, 30.0, 50.0);
        let mut it = picture(1, rect);
        if let Content::Graphic(g) = &mut it.content {
            g.xf = Affine::translate((-100.0, -100.0)) * Affine::scale(100.0);
        }
        let mut candidates = Vec::new();
        picture_leaves(&it, Affine::IDENTITY, 0, &mut candidates);
        assert_eq!(candidates[0].bounds, rect);
    }

    #[test]
    fn pictures_on_other_pages_and_unapplied_parents_are_not_candidates() {
        let mut d = Document::new(&NewDocument { facing_pages: false, pages: 2, ..Default::default() });
        let rect = Rect::new(20.0, 30.0, 30.0, 50.0);
        let mut it = picture(100, rect);
        it.layer = d.layers[0].id;
        d.insert_item(SpreadRef::Doc(1), it.clone(), None).unwrap();
        let pid = d.add_parent("Q", "test", 1, 12.0, designcraft_doc::Margins::uniform(0.0));
        it.id = ItemId(101);
        let parent_index = d.parents.iter().position(|p| p.id == pid).unwrap();
        d.insert_item(SpreadRef::Parent(parent_index), it, None).unwrap();
        assert!(page_pictures(&d, 0).is_empty());
        assert_eq!(page_pictures(&d, 1).len(), 1);
        d.apply_parent(&[0], Some(pid)).unwrap();
        assert_eq!(page_pictures(&d, 0).len(), 1);
        Arc::make_mut(&mut d.spreads[0]).pages[0].overridden.push(ItemId(101));
        assert!(page_pictures(&d, 0).is_empty());
    }
}

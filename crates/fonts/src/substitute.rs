//! Missing-family choices use names as hints, and candidate traits read from real font files.
use crate::kind::{FontClass, classify};

use super::{style_italic, style_weight};

pub(super) struct Candidate<'a> {
    pub family: &'a str,
    pub style: &'a str,
    pub class: FontClass,
    pub width: Option<u16>,
    pub weight: f32,
    pub italic: bool,
}

fn class_hint(family: &str) -> FontClass {
    // These are substitution hints, not recovered metrics for an absent font.
    match family.to_lowercase().as_str() {
        "elephant" | "sylfaen" | "times" | "times new roman" | "cambria" | "cambria math" => FontClass::Serif,
        "calibri" | "calibri light" => FontClass::SansSerif,
        "rockwell" | "rockwell condensed" => FontClass::SlabSerif,
        _ if family.to_lowercase().contains("display") => FontClass::Decorative,
        _ => classify(&[0; 10], 0, false, family),
    }
}

fn class_distance(want: FontClass, got: FontClass) -> u8 {
    use FontClass::*;
    match (want, got) {
        (a, b) if a == b => 0,
        (Serif | SlabSerif, Serif | SlabSerif) => 0,
        (_, Other) => 2,
        (Decorative | Script, Serif | SlabSerif | SansSerif) => 1,
        _ => 3,
    }
}

fn width_distance(family: &str, candidate: Option<u16>) -> u8 {
    let lower = family.to_lowercase();
    let narrow = lower.contains("condensed") || lower.contains("narrow");
    let wide = lower.contains("expanded") || lower.contains("extended");
    if !narrow && !wide {
        // The missing family's actual width is unknown; don't invent a target advance.
        return 0;
    }
    match candidate {
        Some(w) if (narrow && w < 5) || (wide && w > 5) => 0,
        None => 1,
        _ => 2,
    }
}

pub(super) fn choose<'a>(family: &str, style: &str, candidates: impl IntoIterator<Item = Candidate<'a>>) -> Option<(String, String)> {
    let want = class_hint(family);
    if want == FontClass::Other {
        return None;
    }
    let weight =
        if style.eq_ignore_ascii_case("Regular") && family.to_lowercase().contains("black") { style_weight(family) } else { style_weight(style) };
    let italic = style_italic(style);
    let preferred = match want {
        FontClass::Serif | FontClass::SlabSerif => "Source Serif 4",
        FontClass::Monospaced => "JetBrains Mono",
        _ => "Source Sans 3",
    };
    let score = |c: &Candidate<'_>| {
        (
            class_distance(want, c.class),
            width_distance(family, c.width),
            usize::from(c.italic != italic),
            (c.weight - weight).abs(),
            usize::from(c.family != preferred),
        )
    };
    candidates
        .into_iter()
        .min_by(|a, b| {
            let (ac, aw, ai, ag, ap) = score(a);
            let (bc, bw, bi, bg, bp) = score(b);
            (ac, aw, ai)
                .cmp(&(bc, bw, bi))
                .then(ag.total_cmp(&bg))
                .then(ap.cmp(&bp))
                .then_with(|| a.family.cmp(b.family))
                .then_with(|| a.style.cmp(b.style))
        })
        .map(|c| (c.family.to_string(), c.style.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(family: &str, class: FontClass, width: u16) -> Candidate<'_> {
        Candidate { family, style: "Regular", class, width: Some(width), weight: 400.0, italic: false }
    }

    #[test]
    fn serif_and_mono_choose_available_faces_of_their_type() {
        for name in ["Elephant", "Sylfaen", "Rockwell"] {
            let choices = [candidate("Source Sans 3", FontClass::SansSerif, 5), candidate("Source Serif 4", FontClass::Serif, 5)];
            assert_eq!(choose(name, "Regular", choices).unwrap().0, "Source Serif 4");
        }
        let choices = [candidate("Source Sans 3", FontClass::SansSerif, 5), candidate("JetBrains Mono", FontClass::Monospaced, 5)];
        assert_eq!(choose("Missing Mono", "Regular", choices).unwrap().0, "JetBrains Mono");
    }

    #[test]
    fn width_hint_uses_candidate_width_class_without_inventing_original_metrics() {
        let choices = [candidate("Source Serif 4", FontClass::Serif, 5), candidate("Available Narrow Serif", FontClass::Serif, 3)];
        assert_eq!(choose("Rockwell Condensed", "Regular", choices).unwrap().0, "Available Narrow Serif");
        let choices = [candidate("Available Narrow Serif", FontClass::Serif, 3), candidate("Source Serif 4", FontClass::Serif, 5)];
        assert_eq!(choose("Sylfaen", "Regular", choices).unwrap().0, "Source Serif 4", "no stated width: keep the bundled tie preference");
        assert_eq!(width_distance("Missing Narrow", None), 1);
        assert_eq!(width_distance("Missing Expanded", Some(7)), 0);
        assert_eq!(width_distance("Missing Expanded", Some(3)), 2);
    }

    #[test]
    fn names_and_requested_style_keep_sans_and_heavy_faces() {
        let mut bold = candidate("Source Sans 3", FontClass::SansSerif, 5);
        bold.style = "Bold";
        bold.weight = 700.0;
        let choices = [candidate("Source Sans 3", FontClass::SansSerif, 5), bold];
        assert_eq!(choose("Arial Black", "Regular", choices).unwrap().1, "Bold");
        let choices = [candidate("Source Serif 4", FontClass::Serif, 5), candidate("Source Sans 3", FontClass::SansSerif, 5)];
        assert_eq!(choose("Franklin Gothic Book", "Regular", choices).unwrap().0, "Source Sans 3");
    }

    #[test]
    fn display_name_hints_prefer_an_available_decorative_face() {
        let choices = [candidate("Source Sans 3", FontClass::SansSerif, 5), candidate("Available Display", FontClass::Decorative, 5)];
        assert_eq!(choose("Missing Display", "Regular", choices).unwrap().0, "Available Display");
    }

    #[test]
    fn unknown_names_do_not_gain_an_invented_class() {
        assert!(choose("Unavailable Test Family", "Regular", [candidate("Source Serif 4", FontClass::Serif, 5)]).is_none());
        assert!(choose("Missing Sans", "Regular", []).is_none());
    }

    #[test]
    fn bundled_substitutes_preserve_the_missing_original() {
        let db = super::super::FontDb::with_font_dirs(vec![]);
        let scope = db.scoped(0);
        for (original, expected) in [
            ("Elephant", "Source Serif 4"),
            ("Sylfaen", "Source Serif 4"),
            ("Rockwell Condensed", "Source Serif 4"),
            ("Franklin Gothic Book", "Source Sans 3"),
            ("Missing Mono", "JetBrains Mono"),
        ] {
            assert!(!scope.has_family(original));
            assert_eq!(scope.face(original, "Regular").family, expected);
            assert!(!scope.has_family(original), "substitution must not register an alias");
        }
        assert_eq!(scope.face("Arial Black", "Regular").style, "Bold");
        db.add_font(crate::testing::font_with("Elephant", &['A']).unwrap());
        assert_eq!(db.face("Elephant", "Regular").family, "Elephant", "an available original always wins");
    }

    fn narrow_font() -> Vec<u8> {
        let mut font = crate::testing::font_with("Available Narrow Sans", &['A']).unwrap();
        let count = u16::from_be_bytes(font[4..6].try_into().unwrap()) as usize;
        let record = font[12..].chunks_exact(16).take(count).find(|r| r.starts_with(b"OS/2")).unwrap();
        let at = u32::from_be_bytes(record[8..12].try_into().unwrap()) as usize;
        // OS/2 usWidthClass: 3 is condensed. All other tables/traits remain the bundled sans.
        font[at + 6..at + 8].copy_from_slice(&3u16.to_be_bytes());
        font
    }

    #[test]
    fn known_chains_keep_priority_and_otherwise_use_the_bundled_type() {
        let db = super::super::FontDb::with_font_dirs(vec![]);
        for name in ["Times", "Times New Roman", "Cambria", "Cambria Math"] {
            assert_eq!(db.face(name, "Regular").family, "Source Serif 4");
        }
        assert_eq!(db.face("Calibri", "Regular").family, "Source Sans 3");
        for name in ["Caladea", "Carlito", "Liberation Serif"] {
            db.add_font(crate::testing::font_with(name, &['A']).unwrap());
        }
        for (name, expected) in [("Cambria", "Caladea"), ("Calibri", "Carlito"), ("Times New Roman", "Liberation Serif")] {
            assert_eq!(db.face(name, "Regular").family, expected, "available preferred chains keep priority");
        }
    }

    #[test]
    fn loaded_candidate_width_is_read_from_its_font() {
        let db = super::super::FontDb::with_font_dirs(vec![]);
        assert_eq!(db.add_font(narrow_font()), 1);
        assert_eq!(db.face("Arial Narrow", "Regular").family, "Available Narrow Sans");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn installed_candidate_traits_are_retained_by_the_scan() {
        let dir = std::env::temp_dir().join(format!("font-substitute-width-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("narrow.ttf"), narrow_font()).unwrap();
        let db = super::super::FontDb::with_font_dirs(vec![dir.clone()]);
        assert!(!db.is_loaded("Available Narrow Sans"));
        let face = db.face("Arial Narrow", "Regular");
        assert_eq!(face.family, "Available Narrow Sans");
        assert_eq!(face.skrifa().as_ref().and_then(super::super::font_width), Some(3));
        assert!(!db.scoped(0).has_family("Arial Narrow"));
        std::fs::remove_dir_all(dir).unwrap();
    }

    fn table_record(font: &[u8], tag: &[u8; 4]) -> usize {
        let count = u16::from_be_bytes(font[4..6].try_into().unwrap()) as usize;
        (0..count).map(|i| 12 + i * 16).find(|&i| &font[i..i + 4] == tag).unwrap()
    }

    fn table_offset(font: &[u8], tag: &[u8; 4]) -> usize {
        let i = table_record(font, tag);
        u32::from_be_bytes(font[i + 8..i + 12].try_into().unwrap()) as usize
    }

    fn width_style_font(style: &str, width: u16) -> Vec<u8> {
        let mut font =
            crate::testing::font_with_names(&[(1, "Width Style Sans"), (2, style), (16, "Width Style Sans"), (17, style)], &['A']).unwrap();
        let at = table_offset(&font, b"OS/2");
        font[at + 6..at + 8].copy_from_slice(&width.to_be_bytes());
        font
    }

    #[test]
    fn a_condensed_style_survives_resolution_within_a_family() {
        let db = super::super::FontDb::with_font_dirs(vec![]);
        db.add_font(width_style_font("Regular", 5));
        db.add_font(width_style_font("Condensed", 3));
        let face = db.face("Arial Narrow", "Regular");
        assert_eq!((face.family.as_str(), face.style.as_str()), ("Width Style Sans", "Condensed"));
        assert_eq!(db.face("Width Style Sans", "Regular").style, "Regular", "exact requests keep their own style");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn installed_condensed_style_and_post_only_fixed_pitch_keep_their_traits() {
        use crate::kind::FontClass;
        let dir = std::env::temp_dir().join(format!("font-substitute-traits-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("regular.ttf"), width_style_font("Regular", 5)).unwrap();
        std::fs::write(dir.join("condensed.ttf"), width_style_font("Condensed", 3)).unwrap();
        let mut fixed = crate::testing::font_with("Opaque Fixed Face", &['A']).unwrap();
        let os2 = table_offset(&fixed, b"OS/2");
        // Clear sFamilyClass and PANOSE: neither the name nor OS/2 implies monospaced.
        fixed[os2 + 30..os2 + 42].fill(0);
        let post = table_offset(&fixed, b"post");
        let mut header = fixed[post..post + 32].to_vec();
        header[12..16].copy_from_slice(&1u32.to_be_bytes());
        // A large post table must not make the scan miss the fixed-pitch header.
        header.resize(70_000, 0);
        let offset = fixed.len().next_multiple_of(4);
        fixed.resize(offset, 0);
        fixed.extend_from_slice(&header);
        let record = table_record(&fixed, b"post");
        fixed[record + 8..record + 12].copy_from_slice(&(offset as u32).to_be_bytes());
        fixed[record + 12..record + 16].copy_from_slice(&(header.len() as u32).to_be_bytes());
        std::fs::write(dir.join("fixed.ttf"), fixed).unwrap();
        let db = super::super::FontDb::with_font_dirs(vec![dir.clone()]);
        let face = db.face("Arial Narrow", "Regular");
        assert_eq!((face.family.as_str(), face.style.as_str()), ("Width Style Sans", "Condensed"));
        let catalog_class = db.read_catalog().iter().find(|c| c.family == "Opaque Fixed Face").unwrap().class;
        assert_eq!(catalog_class, FontClass::Monospaced);
        assert_eq!(db.face("Opaque Fixed Face", "Regular").kind().class, catalog_class);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

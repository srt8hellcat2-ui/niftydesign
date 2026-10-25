// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Pinned binary fixtures only; no scoring exports are used to identify overset stories.
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn fixture(variable: &str, name: &str) -> Result<PathBuf> {
    std::env::var(variable)?
        .split(':')
        .map(|folder| Path::new(folder).join(name))
        .find(|p| p.is_file())
        .ok_or_else(|| format!("missing {name}").into())
}

#[test]
#[ignore = "needs PUB_REAL_CORPUS and PUB_CORPUS with the pinned fixtures and the measured substitute fonts"]
fn newsletter_overset_stories_are_notes_once_per_chain_and_normal_text_is_not_reported() -> Result<()> {
    let imported = designcraft_pub::import_pub(&std::fs::read(fixture("PUB_REAL_CORPUS", "SampleNewsletter.pub")?)?)?;
    let notes: Vec<_> = imported.notes.iter().filter(|n| n.contains("Some text in this frame does not fit")).collect();
    assert_eq!(notes.len(), 7, "one note per story, not per threaded frame: {notes:?}");
    for beginning in [
        "Living and Learning in Class X",
        "What it means to be an American",
        "Our Excursion to the Zoo",
        "Strictly Jazz",
        "This week in class",
        "On Monday",
        "This week’s reminders",
    ] {
        let found: Vec<_> = notes.iter().filter(|n| n.contains(beginning)).collect();
        assert_eq!(found.len(), 1, "{beginning}: {notes:?}");
        assert!(found[0].starts_with("Page 1,"));
    }
    assert!(!imported.warnings.iter().any(|w| w.contains("Some text in this frame does not fit")));
    let normal = designcraft_pub::import_pub(&std::fs::read(fixture("PUB_CORPUS", "text__base.pub")?)?)?;
    assert!(!normal.notes.iter().any(|n| n.contains("Some text in this frame does not fit")));
    let only_notes = designcraft_pub::import_pub(&std::fs::read(fixture("PUB_CORPUS", "text__size-72.pub")?)?)?;
    assert!(only_notes.warnings.is_empty(), "overset must not become a warning: {:?}", only_notes.warnings);
    assert_eq!(only_notes.notes.iter().filter(|n| n.contains("Some text in this frame does not fit")).count(), 1);
    Ok(())
}

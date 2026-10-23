// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Rendering regression for the external pinned controlled table fixture.
use std::{io::Read, path::Path};

#[test]
#[ignore = "needs PUB_CORPUS containing pinned table__2x2.pub"]
fn imported_table_cells_reach_composition_and_page_pixels() {
    let dirs = std::env::var("PUB_CORPUS").expect("set PUB_CORPUS");
    let path = dirs.split(':').map(|d| Path::new(d).join("table__2x2.pub")).find(|p| p.is_file()).unwrap();
    let mut bytes = Vec::new();
    std::fs::File::open(path).unwrap().take(16 * 1024 * 1024 + 1).read_to_end(&mut bytes).unwrap();
    assert!(bytes.len() <= 16 * 1024 * 1024);
    let document = designcraft_pub::import_pub(&bytes).unwrap().document;
    let cache = designcraft_compose::Cache::new();
    let host = document.stories.values().find(|s| !s.tables.is_empty()).unwrap();
    let composed = cache.get(&document, host.id, None);
    let mut words = Vec::new();
    for frame in &composed.frames {
        for fragment in &frame.tables {
            for cell in &fragment.cells {
                for text_frame in &cell.text.frames {
                    for line in &text_frame.lines {
                        words.push(cell.source[line.range.clone()].to_owned());
                    }
                }
            }
        }
    }
    assert_eq!(words, ["A1", "B1", "A2", "B2"]);
    let image =
        designcraft_render::Renderer::new().render_page(&document, &cache, 0, 1.0, false, &designcraft_render::RenderOptions::default()).unwrap();
    if let Ok(out) = std::env::var("PUB_TABLE_RENDER_OUT") {
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(Path::new(&out).join("table-2x2-page-1.png"), image.to_png()).unwrap();
    }
    assert!(image.pixels.chunks_exact(4).any(|p| p[..3] != [255, 255, 255]), "table text must paint the page");
}

#[test]
#[ignore = "needs pinned PUB_POI_TABLES; renders stay in PUB_TABLE_RENDER_OUT outside tracked docs"]
fn apache_table_render_diagnostics() {
    let root = std::env::var("PUB_POI_TABLES").expect("set PUB_POI_TABLES");
    let out = std::env::var("PUB_TABLE_RENDER_OUT").expect("set private PUB_TABLE_RENDER_OUT");
    std::fs::create_dir_all(&out).unwrap();
    for name in
        ["Sample.pub", "Sample2.pub", "Sample3.pub", "Sample4.pub", "Sample2_2010.pub", "Sample3_2010.pub", "Sample4_2010.pub", "Sample_2010.pub"]
    {
        let mut bytes = Vec::new();
        std::fs::File::open(Path::new(&root).join(name)).unwrap().take(16 * 1024 * 1024 + 1).read_to_end(&mut bytes).unwrap();
        assert!(bytes.len() <= 16 * 1024 * 1024);
        let imported = designcraft_pub::import_pub(&bytes).unwrap();
        assert!(!imported.warnings.iter().any(|w| w.contains("multiple stories/TCD") || w.contains("cell-text association")), "{name}");
        let document = imported.document;
        let cache = designcraft_compose::Cache::new();
        let host = document.stories.values().find(|s| !s.tables.is_empty()).unwrap();
        let composed = cache.get(&document, host.id, None);
        let cell_lines: usize = composed
            .frames
            .iter()
            .flat_map(|f| &f.tables)
            .flat_map(|t| &t.cells)
            .flat_map(|c| &c.text.frames)
            .map(|f| f.lines.iter().filter(|l| !l.glyphs.is_empty()).count())
            .sum();
        let nonempty_cells = host.tables.values().flat_map(|t| &t.cells).filter(|c| !c.text.text.trim().is_empty()).count();
        let table = host.tables.values().next().unwrap();
        assert_eq!((table.nrows(), table.ncols()), (3, 2), "{name}");
        let expected = ["Table on page 2", "Top right", "P2 table left", "P2 table right", "Bottom Left", "Bottom Right"];
        assert_eq!(table.cells.iter().map(|c| c.text.text.as_str()).collect::<Vec<_>>(), expected, "{name}");
        assert_eq!(nonempty_cells, 6, "{name}");
        assert_eq!(cell_lines, 6, "all six cells must reach composition: {name}");

        let fallback_lines: usize = composed.frames.iter().flat_map(|f| &f.lines).filter(|l| !l.glyphs.is_empty()).count();
        println!(
            "{name}: host_frames={} nonempty_cells={nonempty_cells} cell_glyph_lines={cell_lines} retained_text_lines={fallback_lines} overset={:?}",
            composed.frames.len(),
            composed.overset_at
        );
        let pdf = designcraft_pdf::export_pdf(&document, &cache, &designcraft_pdf::PdfOptions::default()).unwrap();
        std::fs::write(Path::new(&out).join(format!("{name}.pdf")), &pdf).unwrap();
        for page in 0..document.page_count() {
            let raster = designcraft_render::pdf_page_png(&pdf, page, 800, &[], None).unwrap();
            assert!(!raster.is_empty(), "{name} PDF page {page}");
            std::fs::write(Path::new(&out).join(format!("{name}-pdf-{}.png", page + 1)), raster).unwrap();
            let image = designcraft_render::Renderer::new()
                .render_page(&document, &cache, page, 1.0, false, &designcraft_render::RenderOptions::default())
                .unwrap();
            std::fs::write(Path::new(&out).join(format!("{name}-{}.png", page + 1)), image.to_png()).unwrap();
        }
    }
}

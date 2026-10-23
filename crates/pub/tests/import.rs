// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Publisher import on a synthetic publication (`tests/data/hello-nifty.pub`, written by our own
//! writer: one text box and one filled rectangle on a US Letter page), plus hostile input. Saving,
//! reopening and exporting it are tested in the engine (`file.rs`, `extension_tests`).

use std::io::{Cursor, Read, Write};

use designcraft_doc::{Content, Document, Item};
use designcraft_pub::{PubError, import_pub, is_pub};

const HELLO: &[u8] = include_bytes!("data/hello-nifty.pub");
/// `hello-nifty.pub` with its rectangle replaced by a WMF picture (`write_wmf_picture_sample`).
const WMF_PICTURE: &[u8] = include_bytes!("data/wmf-picture.pub");

#[test]
#[ignore = "requires PUB_CORPUS with the allowed pubsmith binary fixtures"]
fn real_shape_shadows() {
    let dirs = std::env::var("PUB_CORPUS").unwrap();
    for (name, offset, color) in
        [("shadow__on.pub", 2.0, [0, 0, 0, 255]), ("shadow__offset.pub", 6.0, [0, 0, 0, 255]), ("wordart__shadow.pub", 2.0, [134, 134, 134, 255])]
    {
        let fixture = dirs.split(':').map(|p| std::path::Path::new(p).join(name)).find(|p| p.is_file()).unwrap();
        let imported = import_pub(&std::fs::read(fixture).unwrap()).unwrap();
        assert!(!imported.warnings.iter().any(|w| w.contains("shadow")), "{name}: {:?}", imported.warnings);
        let d = &imported.document;
        let shadows: Vec<_> = items(d).into_iter().filter(|i| i.effects.drop_shadow.on).collect();
        assert_eq!(shadows.len(), 1, "{name}");
        let item = shadows[0];
        let ds = &item.effects.drop_shadow;
        assert_eq!(ds.opacity, 1.0, "{name}");
        assert_eq!((ds.size, ds.spread), (0.0, 0.0));
        assert!((-ds.angle.to_radians().cos() * ds.distance - offset).abs() < 1e-9, "{name}");
        assert!((ds.angle.to_radians().sin() * ds.distance - offset).abs() < 1e-9, "{name}");
        let designcraft_color::SwatchValue::Color { color: actual, .. } = &d.swatch(&ds.color).unwrap().value else { panic!("not a color") };
        assert_eq!(actual.to_rgba8(1.0), color, "{name}");
        assert_eq!(item.opacity, 1.0);
    }
}

#[test]
#[ignore = "requires PUB_CORPUS with the allowed pubsmith binary fixtures"]
fn real_solid_fill_opacity() {
    let dirs = std::env::var("PUB_CORPUS").unwrap();
    let fixture = dirs.split(':').map(|p| std::path::Path::new(p).join("fill__transparency-50.pub")).find(|p| p.is_file()).unwrap();
    let imported = import_pub(&std::fs::read(fixture).unwrap()).unwrap();
    assert!(imported.warnings.is_empty(), "{:?}", imported.warnings);
    let d = &imported.document;
    let transparent: Vec<_> = items(d)
        .into_iter()
        .filter_map(|item| designcraft_color::swatch::resolve_gradient(&d.swatches, &item.fill.swatch).map(|g| (item, g)))
        .collect();
    assert_eq!(transparent.len(), 1);
    let (item, gradient) = transparent[0];
    assert_eq!(gradient.stops.len(), 2);
    for stop in &gradient.stops {
        assert_eq!(stop.opacity, 0.5);
        assert_eq!(stop.color.to_rgba8(1.0), [0, 0, 255, 255]);
    }
    assert_eq!(item.opacity, 1.0);
    assert_ne!(item.stroke.swatch, designcraft_color::swatch::NONE);
    assert!(item.stroke.weight > 0.0);
}

fn items(d: &Document) -> Vec<&Item> {
    d.spreads.iter().flat_map(|s| s.items.iter().map(|i| &**i)).collect()
}

fn text_of(d: &Document, it: &Item) -> Option<String> {
    let Content::Text(t) = &it.content else { return None };
    d.stories.get(&t.story).map(|s| s.text.clone())
}

/// The fixture with its `Contents` stream replaced by `f(contents)`.
fn with_contents(f: impl FnOnce(Vec<u8>) -> Vec<u8>) -> Vec<u8> {
    let mut cf = cfb::CompoundFile::open(Cursor::new(HELLO.to_vec())).unwrap();
    let mut contents = Vec::new();
    cf.open_stream("/Contents").unwrap().read_to_end(&mut contents).unwrap();
    let new = f(contents);
    let mut s = cf.create_stream("/Contents").unwrap();
    s.write_all(&new).unwrap();
    drop(s);
    cf.flush().unwrap();
    cf.into_inner().into_inner()
}

#[test]
fn imports_text_box_and_rectangle_as_editable_objects_nfty_08f72c7b() {
    assert!(is_pub(HELLO));
    let imp = import_pub(HELLO).unwrap();
    let d = &imp.document;
    assert!(imp.warnings.is_empty(), "{:?}", imp.warnings);
    assert_eq!(d.page_count(), 1);
    let page = &d.spreads[0].pages[0];
    assert_eq!((page.width, page.height), (612.0, 792.0));
    let its = items(d);
    assert_eq!(its.len(), 2);

    let text = its.iter().find(|i| i.is_text_frame()).unwrap();
    assert_eq!(text_of(d, text).as_deref(), Some("Hello from NiftyDesign"));
    let b = text.inner_bounds();
    assert!((b.x0 - 72.0).abs() < 0.5 && (b.y0 - 72.0).abs() < 0.5 && (b.width() - 288.0).abs() < 0.5 && (b.height() - 72.0).abs() < 0.5, "{b:?}");

    let rect = its.iter().find(|i| !i.is_text_frame()).unwrap();
    let b = rect.inner_bounds();
    assert!((b.x0 - 72.0).abs() < 0.5 && (b.y0 - 216.0).abs() < 0.5 && (b.width() - 216.0).abs() < 0.5 && (b.height() - 144.0).abs() < 0.5, "{b:?}");
    let sw = d.swatch(&rect.fill.swatch).unwrap();
    let designcraft_color::SwatchValue::Color { color, .. } = &sw.value else { panic!("fill is not a colour: {sw:?}") };
    let [r, g, bl, _] = color.to_rgba8(1.0);
    assert_eq!([r, g, bl], [0x1f, 0x6f, 0xd0]);
}

#[test]
fn rejects_files_that_are_not_publications() {
    assert!(matches!(import_pub(b""), Err(PubError::NotPub(_))));
    assert!(matches!(import_pub(b"%PDF-1.7\n"), Err(PubError::NotPub(_))));
    assert!(!is_pub(b"PK\x03\x04"));
    // A compound file without Publisher's streams (here: Contents with a foreign header).
    let other = with_contents(|_| b"\xd0\xcf\x11\xe0 not publisher".to_vec());
    assert!(matches!(import_pub(&other), Err(PubError::NotPub(_))));
}

#[test]
fn refuses_publisher_2000_and_older_with_a_message() {
    let old = with_contents(|mut c| {
        c[2] = 0x22;
        c
    });
    let Err(PubError::Unsupported(msg)) = import_pub(&old) else { panic!("expected Unsupported") };
    assert!(msg.contains("Publisher 97/98/2000"), "{msg}");
}

#[test]
fn truncated_or_corrupt_input_is_an_error_not_a_panic() {
    for n in [8, 512, 1024, 2048, HELLO.len() / 2, HELLO.len() - 1] {
        let _ = import_pub(&HELLO[..n]);
    }
    let short = with_contents(|c| c[..30.min(c.len())].to_vec());
    assert!(import_pub(&short).is_err());
    // Deterministic byte mutations across the whole file must never panic.
    let mut seed = 0x2545_f491_u32;
    for _ in 0..3000 {
        let mut b = HELLO.to_vec();
        for _ in 0..4 {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let at = seed as usize % b.len();
            b[at] = (seed >> 24) as u8;
        }
        let _ = import_pub(&b);
    }
}

#[test]
fn wmf_pictures_import_as_vector_svg() {
    let imp = import_pub(WMF_PICTURE).unwrap();
    let d = &imp.document;
    assert_eq!(imp.warnings, vec!["Text inside WMF/EMF pictures was left out.".to_string()]);
    let its = items(d);
    assert_eq!(its.len(), 2);
    let pic = its.iter().find(|i| !i.is_text_frame()).unwrap();
    let b = pic.inner_bounds();
    assert!((b.x0 - 72.0).abs() < 0.5 && (b.y0 - 216.0).abs() < 0.5 && (b.width() - 216.0).abs() < 0.5 && (b.height() - 144.0).abs() < 0.5, "{b:?}");
    let Content::Graphic(g) = &pic.content else { panic!("not a picture: {:?}", pic.content) };
    let asset = d.assets.get(&g.asset).unwrap();
    assert_eq!(asset.mime, "image/svg+xml");
    assert!(designcraft_images::svg_tree(&asset.data).is_some());
    let svg = String::from_utf8(asset.data.to_vec()).unwrap();
    assert!(svg.contains("fill=\"#ff0000\"") && svg.contains("fill=\"#0000ff\""), "{svg}");
}

/// Regenerates `tests/data/wmf-picture.pub`: the text box of `hello-nifty.pub` plus, for its
/// second object, a picture frame holding a small WMF (a red triangle on a blue ellipse and a
/// text record), written with our own MS-ODRAW/MS-WMF records.
#[test]
#[ignore = "writes tests/data/wmf-picture.pub"]
fn write_wmf_picture_sample() {
    fn rec(ver: u16, inst: u16, ty: u16, data: &[u8]) -> Vec<u8> {
        let mut r = (ver | inst << 4).to_le_bytes().to_vec();
        r.extend(ty.to_le_bytes());
        r.extend((data.len() as u32).to_le_bytes());
        r.extend(data);
        r
    }
    fn pairs(p: &[(u16, i32)]) -> Vec<u8> {
        let mut d = ((4 + 6 * p.len()) as u32).to_le_bytes().to_vec();
        for (id, v) in p {
            d.extend(id.to_le_bytes());
            d.extend(v.to_le_bytes());
        }
        d
    }
    fn wmf_rec(func: u16, params: &[i16]) -> Vec<u8> {
        let mut r = (3 + params.len() as u32).to_le_bytes().to_vec();
        r.extend(func.to_le_bytes());
        params.iter().for_each(|p| r.extend(p.to_le_bytes()));
        r
    }
    let emu = |pt: i32| pt * 12_700;
    // Page-centre-relative anchor of a frame at (x, y, w, h) pt on the 612 × 792 pt page.
    let anchor = |x: i32, y: i32, w: i32, h: i32| {
        pairs(&[(0x2001, emu(x - 306)), (0x2002, emu(y - 396)), (0x2003, emu(x + w - 306)), (0x2004, emu(y + h - 396))])
    };

    let mut body = Vec::new();
    body.extend(wmf_rec(0x020B, &[0, 0]));
    body.extend(wmf_rec(0x020C, &[200, 300]));
    body.extend(wmf_rec(0x02FC, &[0, 0, 0x00ff, 0])); // blue brush
    body.extend(wmf_rec(0x02FC, &[0, 0x00ff, 0, 0])); // red brush
    body.extend(wmf_rec(0x02FA, &[0, 4, 0, 0, 0])); // black pen, 4 units
    body.extend(wmf_rec(0x012D, &[0]));
    body.extend(wmf_rec(0x012D, &[2]));
    body.extend(wmf_rec(0x0418, &[200, 300, 0, 0]));
    body.extend(wmf_rec(0x012D, &[1]));
    body.extend(wmf_rec(0x0324, &[3, 150, 20, 270, 180, 30, 180]));
    body.extend(wmf_rec(0x0521, &[2, 0x6948, 10, 10]));
    body.extend(wmf_rec(0x0000, &[]));
    let mut wmf = Vec::new();
    [1u16, 9, 0x0300].iter().for_each(|v| wmf.extend(v.to_le_bytes()));
    wmf.extend(((18 + body.len()) as u32 / 2).to_le_bytes());
    wmf.extend(3u16.to_le_bytes());
    wmf.extend(7u32.to_le_bytes());
    wmf.extend(0u16.to_le_bytes());
    wmf.extend(body);

    let packed = miniz_oxide::deflate::compress_to_vec_zlib(&wmf, 9);
    let mut blip = vec![0u8; 16]; // UID
    blip.extend((wmf.len() as u32).to_le_bytes());
    [0i32, 0, 300, 200, emu(216), emu(144)].iter().for_each(|v| blip.extend(v.to_le_bytes()));
    blip.extend((packed.len() as u32).to_le_bytes());
    blip.extend([0x00, 0xFE]);
    blip.extend(packed);
    let blip = rec(0, 0x216, 0xF01B, &blip);
    let mut bse = vec![3u8, 4];
    bse.extend([0u8; 16]);
    bse.extend(0xFFu16.to_le_bytes());
    bse.extend((blip.len() as u32).to_le_bytes());
    bse.extend(1u32.to_le_bytes());
    bse.extend(0u32.to_le_bytes());
    bse.extend([0u8; 4]);
    bse.extend(blip);
    let mut dgg = rec(0, 0, 0xF006, &[0u8; 16]);
    dgg.extend(rec(0xF, 1, 0xF001, &rec(2, 3, 0xF007, &bse)));

    let fsp = |kind: u16, spid: u32, flags: u32| rec(2, kind, 0xF00A, &[spid.to_le_bytes(), flags.to_le_bytes()].concat());
    let mut patriarch = rec(1, 0, 0xF009, &[0u8; 16]);
    patriarch.extend(fsp(0, 1024, 0x5));
    let mut text = fsp(202, 1025, 0xA00);
    text.extend(rec(0, 0, 0xF010, &anchor(72, 72, 288, 72)));
    text.extend(rec(0, 0, 0xF011, &pairs(&[(0x6801, 258)])));
    let mut pic = fsp(75, 1026, 0xA00);
    pic.extend(rec(3, 1, 0xF00B, &[0x4104u16.to_le_bytes().to_vec(), 1u32.to_le_bytes().to_vec()].concat()));
    pic.extend(rec(0, 0, 0xF010, &anchor(72, 216, 216, 144)));
    pic.extend(rec(0, 0, 0xF011, &pairs(&[(0x6801, 259)])));
    let mut spgr = rec(0xF, 0, 0xF004, &patriarch);
    spgr.extend(rec(0xF, 0, 0xF004, &text));
    spgr.extend(rec(0xF, 0, 0xF004, &pic));
    let mut dg = rec(0, 1, 0xF008, &[3u32.to_le_bytes(), 1026u32.to_le_bytes()].concat());
    dg.extend(rec(0xF, 0, 0xF003, &spgr));

    let mut escher = rec(0xF, 0, 0xF000, &dgg);
    escher.extend(0u32.to_le_bytes()); // Publisher's u32 after the container
    escher.extend(rec(0xF, 0, 0xF002, &dg));
    escher.extend(0u32.to_le_bytes());

    let mut cf = cfb::CompoundFile::open(Cursor::new(HELLO.to_vec())).unwrap();
    cf.create_stream("/Escher/EscherStm").unwrap().write_all(&escher).unwrap();
    cf.flush().unwrap();
    let out = cf.into_inner().into_inner();
    std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/wmf-picture.pub"), out).unwrap();
}

// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! A standard Windows metafile as SVG, for callers that hold a bare metafile (the Publisher 97/98/2000 embedded
//! object previews). It runs the same converter the 2002+ importer uses for its WMF pictures and takes no part
//! in the 2002+ import.
use crate::metafile;

/// A metafile converted to SVG, its integer size, and what was left out.
pub struct MetafileSvg {
    pub data: Vec<u8>,
    pub size: (u32, u32),
    pub warnings: Vec<String>,
}

/// Convert a bare metafile (standard or placeable header). `size_emu` is its stored size, used only to scale
/// hairlines. `None` when it holds nothing drawable or is damaged.
pub fn metafile_to_svg(wmf: &[u8], size_emu: (i32, i32)) -> Option<MetafileSvg> {
    let len = u32::try_from(wmf.len()).ok()?;
    // The converter takes an Escher metafile blip: size, bounds, size in EMU, saved length, an
    // "uncompressed" marker and a filter byte, then the metafile.
    let mut blip = Vec::with_capacity(wmf.len().checked_add(34)?);
    blip.extend(len.to_le_bytes());
    blip.extend([0u8; 16]);
    blip.extend(size_emu.0.to_le_bytes());
    blip.extend(size_emu.1.to_le_bytes());
    blip.extend(len.to_le_bytes());
    blip.extend([0xFE, 0xFE]);
    blip.extend(wmf);
    let svg = metafile::to_svg(metafile::BLIP_WMF, &blip)?;
    let (data, size) = limit_size(svg.data, svg.size);
    Some(MetafileSvg { data, size, warnings: svg.warnings.iter().map(|w| (*w).to_string()).collect() })
}

/// The longest side the SVG's own size attributes may have. The converter keeps a large metafile's units as its
/// size (10 794 by 3 763 for a headline), and a PDF writer refuses a drawing whose size in points is that big. The
/// viewBox is left as it is, so the drawing looks the same at any size.
const MAX_SIDE: u32 = 2_000;

/// Scale the `width` and `height` attributes of the SVG's root element (and the size reported with it) down so
/// the longer side is at most [`MAX_SIDE`]. Anything not in the converter's own header form is returned unchanged.
fn limit_size(data: Vec<u8>, size: (u32, u32)) -> (Vec<u8>, (u32, u32)) {
    let longer = size.0.max(size.1);
    if longer <= MAX_SIDE || size.0 == 0 || size.1 == 0 {
        return (data, size);
    }
    let scale = f64::from(MAX_SIDE) / f64::from(longer);
    let side = |v: u32| ((f64::from(v) * scale).round() as u32).max(1);
    let new = (side(size.0), side(size.1));
    let old_attrs = format!("width=\"{}\" height=\"{}\"", size.0, size.1);
    let new_attrs = format!("width=\"{}\" height=\"{}\"", new.0, new.1);
    let text = match String::from_utf8(data) {
        Ok(text) => text,
        Err(e) => return (e.into_bytes(), size),
    };
    let head_end = text.find('>').unwrap_or(0);
    if !text.get(..head_end).is_some_and(|head| head.contains(&old_attrs)) {
        return (text.into_bytes(), size);
    }
    (text.replacen(&old_attrs, &new_attrs, 1).into_bytes(), new)
}

#[cfg(test)]
mod tests {
    use super::{MAX_SIDE, limit_size};

    fn svg(w: u32, h: u32) -> Vec<u8> {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" preserveAspectRatio=\"none\">\n</svg>\n"
        )
        .into_bytes()
    }

    #[test]
    fn a_large_drawing_is_scaled_down_and_keeps_its_view_box_and_proportions() {
        let (data, size) = limit_size(svg(10_794, 3_763), (10_794, 3_763));
        let text = String::from_utf8(data).unwrap();
        assert!(size.0.max(size.1) <= MAX_SIDE && size.0 > size.1, "{size:?}");
        assert!(text.contains(&format!("width=\"{}\" height=\"{}\"", size.0, size.1)), "{text}");
        assert!(text.contains("viewBox=\"0 0 10794 3763\""), "the view box is untouched: {text}");
        let ratio = f64::from(size.0) / f64::from(size.1);
        assert!((ratio - 10_794.0 / 3_763.0).abs() < 0.01, "{ratio}");
    }

    #[test]
    fn a_drawing_within_the_limit_is_left_alone() {
        let before = svg(1_000, 400);
        let (data, size) = limit_size(before.clone(), (1_000, 400));
        assert_eq!((data, size), (before, (1_000, 400)));
    }
}

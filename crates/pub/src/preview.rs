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
    Some(MetafileSvg { data: svg.data, size: svg.size, warnings: svg.warnings.iter().map(|w| (*w).to_string()).collect() })
}

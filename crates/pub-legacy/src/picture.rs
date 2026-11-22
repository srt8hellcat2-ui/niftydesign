// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Pictures of the Publisher 97/98/2000 files: EXPERIMENTAL bitmap extraction.
//!
//! OBSERVATION (docs/pub-format/legacy-minimal-reader.md, "Pictures"): a record of layout index 33 with a
//! prefix of 8 bytes holds, after that prefix, a plain Windows metafile (no placeable header). Its drawing
//! records include one bitmap stretch (`META_STRETCHDIB`, `META_DIBSTRETCHBLT` or `META_SETDIBTODEV`) whose
//! data is an uncompressed packed DIB. The record's parent is the object that owns the picture and carries
//! the rectangle. Only uncompressed 1, 4, 8, 24 and 32 bit DIBs whose size fits exactly are returned; anything
//! else is skipped, never guessed. Text and drawing inside the metafile are not read.
use crate::contents::{Layout, u16_at, u32_at};
use crate::{FrameRect, LegacyError};

/// Most pictures returned, and the most bytes one picture may take.
pub const MAX_PICTURES: usize = 4096;
pub const MAX_PICTURE_BYTES: usize = 128 * 1024 * 1024;

/// The layout index of a picture record and its fixed prefix length.
const PICTURE_INDEX: u16 = 33;
const PICTURE_PREFIX: usize = 8;

/// One bitmap picture: a BMP file, its pixel size, and the object that owns it.
#[derive(Clone, Debug)]
pub struct Picture {
    /// The picture record's id.
    pub record: u16,
    /// The owning object's record id, and its candidate rectangle when the whole box is plausible.
    pub object: u16,
    pub frame: Option<FrameRect>,
    pub width: u32,
    pub height: u32,
    /// A complete BMP file (file header plus the stored DIB).
    pub bmp: Vec<u8>,
}

/// Read every extractable bitmap picture of a legacy file. Pictures that cannot be vouched for are left out.
pub fn read_pictures(bytes: &[u8]) -> Result<Vec<Picture>, LegacyError> {
    let container = crate::open(bytes)?;
    let layout = Layout::parse(&container.contents, &container.clsid)?;
    let page = layout.size_candidate().0.map(|s| (s.raw_width, s.raw_height));
    let mut out = Vec::new();
    let mut total = 0usize;
    for rec in &layout.records {
        if rec.index != PICTURE_INDEX || rec.h != PICTURE_PREFIX || out.len() >= MAX_PICTURES {
            continue;
        }
        let Some(body) = layout.data.get(rec.r + PICTURE_PREFIX..rec.e) else { continue };
        let Some(dib) = find_dib(body) else { continue };
        let Some(pic) = to_picture(rec.id, rec.parent, dib, page.and_then(|p| layout.frame_rect(rec.parent, p))) else { continue };
        total = total.saturating_add(pic.bmp.len());
        if total > MAX_PICTURE_BYTES {
            break;
        }
        out.push(pic);
    }
    Ok(out)
}

/// The packed DIB of the first bitmap-drawing record in a standard metafile, if the metafile is well formed.
fn find_dib(wmf: &[u8]) -> Option<&[u8]> {
    if !matches!(u16_at(wmf, 0)?, 1 | 2) {
        return None;
    }
    let header_words = usize::from(u16_at(wmf, 2)?);
    if header_words < 9 {
        return None;
    }
    let mut at = header_words.checked_mul(2)?;
    // Records are (size in words: u32, function: u16, parameters); the end record has function 0.
    for _ in 0..4096 {
        let words = usize::try_from(u32_at(wmf, at)?).ok()?;
        let func = u16_at(wmf, at.checked_add(4)?)?;
        let len = words.checked_mul(2)?;
        if len < 6 || func == 0 {
            return None;
        }
        let end = at.checked_add(len)?;
        // Offset of the DIB inside each bitmap record: 6 bytes of header plus the fixed parameters.
        let dib_at = match func {
            0x0F43 => Some(6 + 22), // META_STRETCHDIB
            0x0B41 => Some(6 + 20), // META_DIBSTRETCHBLT
            0x0D33 => Some(6 + 18), // META_SETDIBTODEV
            _ => None,
        };
        if let Some(off) = dib_at {
            return wmf.get(at.checked_add(off)?..end);
        }
        at = end;
    }
    None
}

/// Validate an uncompressed packed DIB and wrap it as a BMP file.
fn to_picture(record: u16, object: u16, dib: &[u8], frame: Option<FrameRect>) -> Option<Picture> {
    let header = usize::try_from(u32_at(dib, 0)?).ok()?;
    if header != 40 {
        return None;
    }
    let width = i32::try_from(u32_at(dib, 4)?).ok().filter(|w| (1..=20_000).contains(w))? as u32;
    let height_raw = u32_at(dib, 8)? as i32;
    let height = height_raw.unsigned_abs();
    if !(1..=20_000).contains(&height) || u16_at(dib, 12)? != 1 {
        return None;
    }
    let bits = u16_at(dib, 14)?;
    if !matches!(bits, 1 | 4 | 8 | 24 | 32) || u32_at(dib, 16)? != 0 {
        return None;
    }
    let mut colors = usize::try_from(u32_at(dib, 32)?).ok()?;
    if colors == 0 && bits <= 8 {
        colors = 1usize << bits;
    }
    if colors > 256 {
        return None;
    }
    let stride = (usize::try_from(width).ok()?.checked_mul(usize::from(bits))?.checked_add(31)? / 32).checked_mul(4)?;
    let pixels = stride.checked_mul(usize::try_from(height).ok()?)?;
    let needed = header.checked_add(colors.checked_mul(4)?)?.checked_add(pixels)?;
    // The stored bitmap must cover exactly what its header promises (extra trailing bytes are ignored).
    if dib.len() < needed || needed > MAX_PICTURE_BYTES {
        return None;
    }
    let dib = dib.get(..needed)?;
    let offset = 14usize.checked_add(header)?.checked_add(colors * 4)?;
    let total = dib.len().checked_add(14)?;
    let mut bmp = Vec::with_capacity(total);
    bmp.extend(b"BM");
    bmp.extend(u32::try_from(total).ok()?.to_le_bytes());
    bmp.extend([0u8; 4]);
    bmp.extend(u32::try_from(offset).ok()?.to_le_bytes());
    bmp.extend(dib);
    Some(Picture { record, object, frame, width, height, bmp })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dib(w: u32, h: u32, bits: u16, compression: u32) -> Vec<u8> {
        let stride = (w as usize * bits as usize).div_ceil(32) * 4;
        let mut v = Vec::new();
        v.extend(40u32.to_le_bytes());
        v.extend(w.to_le_bytes());
        v.extend(h.to_le_bytes());
        v.extend(1u16.to_le_bytes());
        v.extend(bits.to_le_bytes());
        v.extend(compression.to_le_bytes());
        v.extend([0u8; 20]);
        v.extend(vec![0x7f; stride * h as usize]);
        v
    }

    /// A standard metafile with one META_STRETCHDIB record around `dib`.
    fn wmf(dib: &[u8]) -> Vec<u8> {
        let mut v = vec![1, 0, 9, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let words = (6 + 22 + dib.len()).div_ceil(2) as u32;
        v.extend(words.to_le_bytes());
        v.extend(0x0F43u16.to_le_bytes());
        v.extend([0u8; 22]);
        v.extend(dib);
        if v.len() % 2 == 1 {
            v.push(0);
        }
        v.extend([3, 0, 0, 0, 0, 0]); // end record
        v
    }

    #[test]
    fn a_24_bit_bitmap_in_a_metafile_becomes_a_bmp_file() {
        let d = dib(3, 2, 24, 0);
        let found = find_dib(&wmf(&d)).unwrap();
        let p = to_picture(0x44, 0x43, found, None).unwrap();
        assert_eq!((p.width, p.height), (3, 2));
        assert_eq!(&p.bmp[..2], b"BM");
        assert_eq!(p.bmp.len(), 14 + d.len());
    }

    #[test]
    fn compressed_or_truncated_bitmaps_are_left_out() {
        assert!(to_picture(1, 2, &dib(3, 2, 24, 1), None).is_none());
        let d = dib(3, 2, 24, 0);
        assert!(to_picture(1, 2, &d[..d.len() - 1], None).is_none());
        assert!(to_picture(1, 2, &dib(0, 2, 24, 0), None).is_none());
        assert!(to_picture(1, 2, &dib(3, 2, 16, 0), None).is_none());
    }

    #[test]
    fn a_metafile_without_a_bitmap_record_or_a_bad_header_yields_nothing() {
        assert!(find_dib(&[1, 0, 9, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0]).is_none());
        assert!(find_dib(&[9, 9, 9, 9]).is_none());
        assert!(find_dib(&[]).is_none());
    }
}

/// A stored presentation of an embedded object: the object's drawing as a standard metafile.
///
/// OBSERVATION (docs/pub-format/legacy-minimal-reader.md, "Embedded object previews"): a record of layout
/// index 34 (prefix 8) whose bytes at +8 are `4D 4F` ("MO") names, in the u16 at +10, the storage
/// `/Objects/Object N`; its parent is the owning object (rectangle at +0x06). That storage holds
/// presentation streams (`\x02OlePres000`, `\x02OlePres001`): a presentation header followed by a metafile.
#[derive(Clone, Debug)]
pub struct ObjectPreview {
    /// The `MO` record's id, and the owning object's id with its candidate rectangle.
    pub record: u16,
    pub object: u16,
    pub frame: Option<FrameRect>,
    /// The `N` of `/Objects/Object N`.
    pub number: u16,
    /// Stored size in 1/100 mm (HIMETRIC).
    pub width: u32,
    pub height: u32,
    /// The metafile (standard header, no placeable header).
    pub wmf: Vec<u8>,
}

/// Most metafile bytes read per object, and in all.
pub const MAX_PREVIEW_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PREVIEWS: usize = 1024;

/// Read the metafile previews of embedded objects. Objects whose presentation cannot be vouched for are left out.
pub fn read_object_previews(bytes: &[u8]) -> Result<Vec<ObjectPreview>, LegacyError> {
    let container = crate::open(bytes)?;
    let layout = Layout::parse(&container.contents, &container.clsid)?;
    let page = layout.size_candidate().0.map(|s| (s.raw_width, s.raw_height));
    let mut cf = crate::open_cf(bytes)?;
    let mut out = Vec::new();
    let mut total = 0usize;
    for rec in &layout.records {
        if rec.index != 34 || rec.h != 8 || out.len() >= MAX_PREVIEWS {
            continue;
        }
        let Some(body) = layout.data.get(rec.r..rec.e) else { continue };
        if body.get(8..10) != Some(&[0x4d, 0x4f]) {
            continue;
        }
        let Some(number) = u16_at(body, 10) else { continue };
        let Some(frame_page) = page else { continue };
        let Some((width, height, wmf)) = (0..2).find_map(|k| presentation(&mut cf, number, k)) else { continue };
        total = total.saturating_add(wmf.len());
        if total > MAX_PREVIEW_BYTES * 8 {
            break;
        }
        out.push(ObjectPreview { record: rec.id, object: rec.parent, frame: layout.frame_rect(rec.parent, frame_page), number, width, height, wmf });
    }
    Ok(out)
}

/// One presentation stream of `/Objects/Object N`: (width, height, metafile bytes) when it is a well-formed
/// metafile presentation.
fn presentation(cf: &mut crate::Cf<'_>, number: u16, which: usize) -> Option<(u32, u32, Vec<u8>)> {
    use std::io::Read;
    let path = format!("/Objects/Object {number}/\u{2}OlePres00{which}");
    if !cf.is_stream(&path) {
        return None;
    }
    let declared = usize::try_from(cf.entry(&path).ok()?.len()).ok()?;
    if declared > MAX_PREVIEW_BYTES {
        return None;
    }
    let mut d = Vec::new();
    cf.open_stream(&path).ok()?.take(MAX_PREVIEW_BYTES as u64 + 1).read_to_end(&mut d).ok()?;
    // Clipboard format marker (-1) and format 3 (metafile), then the target-device block (its size includes
    // its own 4 bytes), then aspect, index, flags, reserved, width, height and the metafile length.
    if u32_at(&d, 0)? != u32::MAX || u32_at(&d, 4)? != 3 {
        return None;
    }
    let device = usize::try_from(u32_at(&d, 8)?).ok()?;
    if !(4..=65_536).contains(&device) {
        return None;
    }
    let head = 12usize.checked_add(device - 4)?;
    let (width, height) = (u32_at(&d, head.checked_add(16)?)?, u32_at(&d, head.checked_add(20)?)?);
    let size = usize::try_from(u32_at(&d, head.checked_add(24)?)?).ok()?;
    let start = head.checked_add(28)?;
    let wmf = d.get(start..start.checked_add(size)?)?;
    if size < 18 || !matches!(u16_at(wmf, 0)?, 1 | 2) || width == 0 || height == 0 || width > 4_000_000 || height > 4_000_000 {
        return None;
    }
    Some((width, height, wmf.to_vec()))
}

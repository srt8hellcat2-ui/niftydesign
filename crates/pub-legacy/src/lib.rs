// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! EXPERIMENTAL text-only reader for Microsoft Publisher 97, 98 and 2000 files.
//!
//! It follows docs/pub-format/legacy-minimal-reader.md and nothing else: it recovers linked text
//! segments and a raw root-size candidate, and creates no frames, pages or layout. It is kept
//! apart from the 2002+ importer (`designcraft-pub`) and shares no code with it. Every check in
//! that contract is a gate here (G01..G09, AB01..AB08, C01..C06, O01); a file that fails a gate
//! is refused or partly recovered with a warning that names the format, never guessed at.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

mod cmap;
mod contents;
mod text;

use std::fmt;
use std::io::Read;

/// Files and extracted streams above this size are refused (an operational limit, not a format fact).
pub const MAX_BYTES: u64 = 32 * 1024 * 1024;
/// Most recovered segments or story ids.
pub const MAX_SEGMENTS: usize = 65_535;
/// Most Quill directory blocks followed.
pub const MAX_QUILL_BLOCKS: usize = 64;

/// The structural family a file was admitted as. The labels are provisional, not saving versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    A,
    B,
    C,
}

impl Family {
    pub fn label(self) -> &'static str {
        match self {
            Family::A => "experimental legacy A",
            Family::B => "experimental legacy B",
            Family::C => "experimental legacy C",
        }
    }
}

/// Why a whole file was refused.
#[derive(Debug, PartialEq, Eq)]
pub enum LegacyError {
    /// Not a legacy Publisher compound file at all.
    NotLegacy(String),
    /// Damaged: a structure contradicts itself or runs past its bounds.
    Malformed(String),
    /// A profile the contract does not describe.
    Unsupported(String),
    /// An operational limit was reached.
    Limit(String),
}

impl fmt::Display for LegacyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LegacyError::NotLegacy(m) => write!(f, "not an experimental legacy Publisher file: {m}"),
            LegacyError::Malformed(m) => write!(f, "damaged legacy Publisher file: {m}"),
            LegacyError::Unsupported(m) => write!(f, "unsupported legacy Publisher profile: {m}"),
            LegacyError::Limit(m) => write!(f, "legacy Publisher reader limit: {m}"),
        }
    }
}

impl std::error::Error for LegacyError {}

/// Where a segment's text came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// A/B: a Quill story id, its UTF-16 unit range in TEXT, and every Contents record that links to it
    /// (storage order only; neither reading order nor threading).
    Story { id: u32, units: std::ops::Range<usize>, records: Vec<u16> },
    /// C: a directory record id and its byte range in the Contents text buffer (relative to its start).
    Record { id: u16, bytes: std::ops::Range<usize> },
}

/// One recovered piece of text. `text` is `None` when the bytes could not be decoded safely
/// (C segments with bytes at or above 0x80: the code page is not established).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    pub source: Source,
    pub text: Option<String>,
    pub encoding_unknown: bool,
}

/// The root record's raw size pair. Units are NOT verified.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeCandidate {
    pub raw_width: u32,
    pub raw_height: u32,
    /// INFERRED, not verified: raw / 36000 millimetres.
    pub inferred_mm: (f64, f64),
}

/// How much of the text store was recovered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// Every consumed link and segment passed.
    Complete,
    /// Some segments were withheld; see the warnings.
    Partial,
    /// A validated store that holds no stories. This says nothing about the rest of the publication.
    EmptyKnownStore,
    /// The text path was refused; only the size candidate (if any) survives.
    TextRefused,
}

/// The result of reading a legacy file: text, provenance, a size candidate and diagnostics.
#[derive(Clone, Debug)]
pub struct LegacyText {
    pub family: Family,
    pub segments: Vec<Segment>,
    pub size: Option<SizeCandidate>,
    pub scope: Scope,
    pub warnings: Vec<String>,
}

/// Quick check for the legacy Contents discriminator without reading the whole file.
pub fn looks_legacy(bytes: &[u8]) -> bool {
    open(bytes).is_ok_and(|c| c.contents.get(..4) == Some(&[0xe8, 0xac, 0x22, 0x00]))
}

pub(crate) struct Container {
    pub contents: Vec<u8>,
    pub clsid: String,
}

const CFB_SIGNATURE: [u8; 8] = [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];

type Cf<'a> = cfb::CompoundFile<std::io::Cursor<&'a [u8]>>;

fn open_cf(bytes: &[u8]) -> Result<Cf<'_>, LegacyError> {
    if bytes.len() as u64 > MAX_BYTES {
        return Err(LegacyError::Limit(format!("the file is larger than {MAX_BYTES} bytes")));
    }
    if !bytes.starts_with(&CFB_SIGNATURE) {
        return Err(LegacyError::NotLegacy("not a compound file".into()));
    }
    let cf = cfb::CompoundFile::open(std::io::Cursor::new(bytes)).map_err(|e| LegacyError::Malformed(format!("compound file: {e}")))?;
    if cf.version() != cfb::Version::V3 {
        return Err(LegacyError::Unsupported("only version 3 compound files are admitted".into()));
    }
    Ok(cf)
}

/// Read one stream within the byte budget; the delivered length must equal the declared one.
fn read_stream(cf: &mut Cf<'_>, path: &str) -> Result<Option<Vec<u8>>, LegacyError> {
    if !cf.is_stream(path) {
        return Ok(None);
    }
    let declared = cf.entry(path).map_err(|e| LegacyError::Malformed(format!("stream {path}: {e}")))?.len();
    if declared > MAX_BYTES {
        return Err(LegacyError::Limit(format!("stream {path} is larger than {MAX_BYTES} bytes")));
    }
    let s = cf.open_stream(path).map_err(|e| LegacyError::Malformed(format!("stream {path}: {e}")))?;
    let mut out = Vec::new();
    s.take(MAX_BYTES + 1).read_to_end(&mut out).map_err(|e| LegacyError::Malformed(format!("stream {path}: {e}")))?;
    if out.len() as u64 != declared {
        return Err(LegacyError::Malformed(format!("stream {path} delivered {} bytes but declares {declared}", out.len())));
    }
    Ok(Some(out))
}

/// G01: a bounded compound-file read of root Contents; never carves strings from a failed container.
pub(crate) fn open(bytes: &[u8]) -> Result<Container, LegacyError> {
    let mut cf = open_cf(bytes)?;
    let clsid = cf.root_entry().clsid().to_string().to_ascii_lowercase();
    let contents = read_stream(&mut cf, "/Contents")?.ok_or_else(|| LegacyError::NotLegacy("no Contents stream".into()))?;
    Ok(Container { contents, clsid })
}

/// The Quill text stream, read only once the family is known to use it.
fn quill_stream(bytes: &[u8]) -> Result<Option<Vec<u8>>, LegacyError> {
    let mut cf = open_cf(bytes)?;
    read_stream(&mut cf, "/Quill/QuillSub/CONTENTS")
}

/// Read a legacy file. Refuses what it cannot vouch for; see [`LegacyText::scope`].
pub fn read_legacy(bytes: &[u8]) -> Result<LegacyText, LegacyError> {
    let container = open(bytes)?;
    let layout = contents::Layout::parse(&container.contents, &container.clsid)?;
    let family = layout.family;
    let mut warnings = layout.warnings.clone();
    let (size, size_warning) = layout.size_candidate();
    warnings.extend(size_warning);
    let outcome = match family {
        Family::A | Family::B => match quill_stream(bytes) {
            Ok(q) => text::read_quill(&layout, q.as_deref()),
            Err(e) => Err(format!("the Quill stream could not be read: {e}")),
        },
        Family::C => {
            // Family C keeps its text in Contents; an alternate store is warned about, never followed.
            if let Ok(Some(_)) = quill_stream(bytes) {
                warnings.push(format!("{}: an unexpected Quill text store is present and was not read.", family.label()));
            }
            cmap::read_map(&layout)
        }
    };
    let (segments, scope, text_warnings) = match outcome {
        Ok(ok) => (ok.segments, ok.scope, ok.warnings),
        Err(why) => (Vec::new(), Scope::TextRefused, vec![format!("{}: the text path was refused: {why}", family.label())]),
    };
    warnings.extend(text_warnings);
    warnings.push(format!("{}: text and a size candidate only; frames, pages, fonts and formatting are not recovered.", family.label()));
    Ok(LegacyText { family, segments, size, scope, warnings })
}

/// What a text path returns.
pub(crate) struct Recovered {
    pub segments: Vec<Segment>,
    pub scope: Scope,
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests;

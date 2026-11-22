// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Single-byte text for families C and D. INFERENCE (docs/pub-format/legacy-minimal-reader.md, "Code page"): the
//! bytes at or above 0x80 read as Windows-1252 in every donated file that has them. That is not confirmed, so a
//! segment decoded this way is reported as inferred, and a byte that Windows-1252 leaves undefined keeps the
//! segment undecoded.

/// Windows-1252 for 0x80..=0x9f; `None` where the code page defines nothing.
const HIGH: [Option<char>; 32] = [
    Some('\u{20ac}'),
    None,
    Some('\u{201a}'),
    Some('\u{0192}'),
    Some('\u{201e}'),
    Some('\u{2026}'),
    Some('\u{2020}'),
    Some('\u{2021}'),
    Some('\u{02c6}'),
    Some('\u{2030}'),
    Some('\u{0160}'),
    Some('\u{2039}'),
    Some('\u{0152}'),
    None,
    Some('\u{017d}'),
    None,
    None,
    Some('\u{2018}'),
    Some('\u{2019}'),
    Some('\u{201c}'),
    Some('\u{201d}'),
    Some('\u{2022}'),
    Some('\u{2013}'),
    Some('\u{2014}'),
    Some('\u{02dc}'),
    Some('\u{2122}'),
    Some('\u{0161}'),
    Some('\u{203a}'),
    Some('\u{0153}'),
    None,
    Some('\u{017e}'),
    Some('\u{0178}'),
];

pub(crate) enum Decoded {
    /// Every byte is ASCII: exact.
    Ascii(String),
    /// Decoded as Windows-1252 (inferred, not confirmed).
    Inferred(String),
    /// A byte the code page leaves undefined.
    Unknown,
}

pub(crate) fn decode(bytes: &[u8]) -> Decoded {
    if bytes.is_ascii() {
        return Decoded::Ascii(bytes.iter().map(|b| char::from(*b)).collect());
    }
    let mut out = String::with_capacity(bytes.len());
    for b in bytes {
        match *b {
            0x00..=0x7f | 0xa0..=0xff => out.push(char::from(*b)),
            high => match HIGH.get(usize::from(high) - 0x80).copied().flatten() {
                Some(c) => out.push(c),
                None => return Decoded::Unknown,
            },
        }
    }
    Decoded::Inferred(out)
}

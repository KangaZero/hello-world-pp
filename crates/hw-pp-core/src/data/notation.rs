//! Text notations: renders a string as bytes, code units, escapes, encodings and more.

use std::fmt::{self, Write as _};

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::data::macros::name_enum;

/// RFC 4648 §4 alphabet.
const BASE64_STANDARD: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// RFC 4648 §5 alphabet, safe in URLs and file names.
const BASE64_URL_SAFE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// RFC 4648 §6 alphabet.
const BASE32_STANDARD: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

name_enum! {
    /// A way to render text, applied by [`Notation::render`].
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub enum Notation: ParseNotationError("notation") {
        // Plain text
        #[default]
        Utf8 => "UTF8" | "UTF 8",

        // Radix, one group per UTF-8 byte (`std::fmt` traits)
        Binary => "Binary",
        Octal => "Octal",
        Decimal => "Decimal",
        LowerHex => "Lower Hex",
        UpperHex => "Upper Hex",

        // Unicode units
        CodePoints => "Code Points" | "Code Point",
        Utf16 => "UTF16" | "UTF 16",

        // Escapes (std escape iterators, `write!` or `escape_bytes`)
        RustEscape => "Rust Escape",
        RustLiteral => "Rust Literal",
        ByteEscape => "Byte Escape",
        JsEscape => "JS Escape",
        CssEscape => "CSS Escape",
        HtmlDecimalEntities => "HTML Decimal Entities",
        HtmlHexEntities => "HTML Hex Entities",
        PercentEncoding => "Percent Encoding",

        // Bit regrouping and byte escaping
        Base64 => "Base64" | "Base 64",
        Base64Url => "Base64 URL" | "Base 64 URL",
        Base32 => "Base32" | "Base 32",
        QuotedPrintable => "Quoted Printable",

        // Byte-to-glyph mapping
        // Not the Braille script in human_languages: raw 8-dot byte cells.
        Braille => "Braille",

        // Unicode normalization forms (`unicode-normalization`)
        // Nfc => "NFC",
        Nfd => "NFD",
        Nfkc => "NFKC",
        Nfkd => "NFKD",

        // Segmentation (`unicode-segmentation`)
        Graphemes => "Graphemes" | "Grapheme",
    }
}

impl Notation {
    /// `text` rendered in this notation.
    #[must_use]
    pub fn render(self, text: &str) -> String {
        match self {
            Self::Utf8 => text.to_owned(),
            Self::Binary => str_to_binary(text),
            Self::Octal => str_to_octal(text),
            Self::Decimal => str_to_decimal(text),
            Self::LowerHex => str_to_lower_hex(text),
            Self::UpperHex => str_to_upper_hex(text),
            Self::CodePoints => str_to_code_points(text),
            Self::Utf16 => str_to_utf16(text),
            Self::RustEscape => str_to_rust_escape(text),
            Self::RustLiteral => str_to_rust_literal(text),
            Self::ByteEscape => str_to_byte_escape(text),
            Self::JsEscape => str_to_js_escape(text),
            Self::CssEscape => str_to_css_escape(text),
            Self::HtmlDecimalEntities => str_to_html_decimal_entities(text),
            Self::HtmlHexEntities => str_to_html_hex_entities(text),
            Self::PercentEncoding => str_to_percent_encoding(text),
            Self::Base64 => str_to_base64(text),
            Self::Base64Url => str_to_base64_url(text),
            Self::Base32 => str_to_base32(text),
            Self::QuotedPrintable => str_to_quoted_printable(text),
            Self::Braille => str_to_braille_patterns(text),
            // Self::Nfc => str_to_nfc(text),
            Self::Nfd => str_to_nfd(text),
            Self::Nfkc => str_to_nfkc(text),
            Self::Nfkd => str_to_nfkd(text),
            Self::Graphemes => str_to_graphemes(text),
        }
    }

    /// Display names of every notation, e.g. `"Base64"`.
    #[must_use]
    pub fn all_as_str() -> Vec<&'static str> {
        Notation::ALL
            .iter()
            .map(|notation| notation.as_str())
            .collect()
    }
}

/// Writes every unit with `write_unit` into one buffer, `separator` between units.
fn join_units<T>(
    units: impl IntoIterator<Item = T>,
    separator: &str,
    write_unit: impl Fn(&mut String, T) -> fmt::Result,
) -> String {
    let mut out = String::new();
    for (index, unit) in units.into_iter().enumerate() {
        if index > 0 {
            out.push_str(separator);
        }
        write_unit(&mut out, unit).expect("writing to a String never fails");
    }
    out
}

/// `"Hé"` → `"01001000 11000011 10101001"` (`{:08b}` per UTF-8 byte).
fn str_to_binary(text: &str) -> String {
    join_units(text.bytes(), " ", |out, byte| write!(out, "{byte:08b}"))
}

/// `"Hé"` → `"110 303 251"` (`{:03o}` per UTF-8 byte).
fn str_to_octal(text: &str) -> String {
    join_units(text.bytes(), " ", |out, byte| write!(out, "{byte:03o}"))
}

/// `"Hé"` → `"72 195 169"` (`{}` per UTF-8 byte).
fn str_to_decimal(text: &str) -> String {
    join_units(text.bytes(), " ", |out, byte| write!(out, "{byte}"))
}

/// `"Hé"` → `"48 c3 a9"` (`{:02x}` per UTF-8 byte).
fn str_to_lower_hex(text: &str) -> String {
    join_units(text.bytes(), " ", |out, byte| write!(out, "{byte:02x}"))
}

/// `"Hé"` → `"48 C3 A9"` (`{:02X}` per UTF-8 byte).
fn str_to_upper_hex(text: &str) -> String {
    join_units(text.bytes(), " ", |out, byte| write!(out, "{byte:02X}"))
}

/// `"Hé"` → `"U+0048 U+00E9"` (`{:04X}` per `char`).
fn str_to_code_points(text: &str) -> String {
    join_units(text.chars(), " ", |out, character| {
        write!(out, "U+{:04X}", u32::from(character))
    })
}

/// `"H😀"` → `"0048 d83d de00"` (`encode_utf16`, surrogate pairs included).
fn str_to_utf16(text: &str) -> String {
    join_units(text.encode_utf16(), " ", |out, u| write!(out, "{u:04x}"))
}

/// `"Hé"` → `r"\u{48}\u{e9}"` (`str::escape_unicode`).
fn str_to_rust_escape(text: &str) -> String {
    text.escape_unicode().collect()
}

/// `"Hé\n"` → `r"H\u{e9}\n"` (`str::escape_default`).
fn str_to_rust_literal(text: &str) -> String {
    text.escape_default().collect()
}

/// `"Hé"` → `r"H\xc3\xa9"` (`<[u8]>::escape_ascii`).
fn str_to_byte_escape(text: &str) -> String {
    text.as_bytes().escape_ascii().to_string()
}

/// `"H😀"` → `r"\u0048\uD83D\uDE00"` (`encode_utf16` + `{:04X}`).
fn str_to_js_escape(text: &str) -> String {
    join_units(text.encode_utf16(), "", |out, unit| {
        write!(out, "\\u{unit:04X}")
    })
}

/// `"Hé"` → `r"\48\E9"` (`{:X}` per `char`).
fn str_to_css_escape(text: &str) -> String {
    join_units(text.chars(), "", |out, character| {
        write!(out, "\\{:X}", u32::from(character))
    })
}

/// `"Hé"` → `"&#72;&#233;"`.
fn str_to_html_decimal_entities(text: &str) -> String {
    join_units(text.chars(), "", |out, character| {
        write!(out, "&#{};", u32::from(character))
    })
}

/// `"Hé"` → `"&#x48;&#xE9;"`.
fn str_to_html_hex_entities(text: &str) -> String {
    join_units(text.chars(), "", |out, character| {
        write!(out, "&#x{:X};", u32::from(character))
    })
}

/// RFC 3986 §2.3 unreserved characters, never percent-encoded.
fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

/// RFC 2045 §6.7 literal representation: printable ASCII except `=`, plus space and tab.
fn is_quoted_printable_literal(byte: u8) -> bool {
    matches!(byte, b'!'..=b'<' | b'>'..=b'~' | b' ' | b'\t')
}

/// Keeps bytes where `is_literal` holds; writes the rest as `escape` + `{:02X}`.
fn escape_bytes(text: &str, escape: char, is_literal: impl Fn(u8) -> bool) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if is_literal(byte) {
            out.push(char::from(byte));
        } else {
            write!(out, "{escape}{byte:02X}").expect("writing to a String never fails");
        }
    }
    out
}

/// `"H é"` → `"H%20%C3%A9"` (RFC 3986; unreserved = ASCII alphanumeric + `-._~`).
fn str_to_percent_encoding(text: &str) -> String {
    escape_bytes(text, '%', is_unreserved)
}

/// RFC 4648 base64 of `bytes`: 3 bytes (24 bits, LCM of 8 and 6) → 4 characters, `=`-padded.
fn encode_base64(bytes: &[u8], alphabet: &[u8; 64]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let byte_at = |index: usize| chunk.get(index).copied().map_or(0, u32::from);
        let glued_bytes = byte_at(0) << 16 | byte_at(1) << 8 | byte_at(2);

        let meaningful = (chunk.len() * 8).div_ceil(6);

        // 24 bits / 6 = 4 characters per chunk
        for slot in 0..4 {
            if slot < meaningful {
                // Masked to 6 bits (0..=63), so the cast can't truncate.
                let index = ((glued_bytes >> (18 - 6 * slot)) & 0b11_1111) as usize;
                out.push(char::from(alphabet[index]));
            } else {
                out.push('=');
            }
        }
    }

    out
}

/// `"Hé"` → `"SMOp"` (RFC 4648 §4, `+/` alphabet, `=` padding).
fn str_to_base64(text: &str) -> String {
    encode_base64(text.as_bytes(), BASE64_STANDARD)
}

/// `"~~~"` → `"fn5-"` (RFC 4648 §5, `-_` alphabet, `=` padding kept).
fn str_to_base64_url(text: &str) -> String {
    encode_base64(text.as_bytes(), BASE64_URL_SAFE)
}

/// Like [`encode_base64`] with 5-bit groups: 5 bytes (40 bits, LCM of 8 and 5) → 8 characters.
fn encode_base32(bytes: &[u8], alphabet: &[u8; 32]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(5) * 8);

    for chunk in bytes.chunks(5) {
        let byte_at = |index: usize| chunk.get(index).copied().map_or(0, u64::from);
        let glued_bytes =
            byte_at(0) << 32 | byte_at(1) << 24 | byte_at(2) << 16 | byte_at(3) << 8 | byte_at(4);

        let meaningful = (chunk.len() * 8).div_ceil(5);

        // 40 bits / 5 = 8 characters per chunk
        for slot in 0..8 {
            if slot < meaningful {
                // Masked to 5 bits (0..=31), so the cast can't truncate.
                let index = ((glued_bytes >> (35 - 5 * slot)) & 0b1_1111) as usize;
                out.push(char::from(alphabet[index]));
            } else {
                out.push('=');
            }
        }
    }

    out
}

/// `"Hé"` → `"JDB2S==="` (RFC 4648 §6, `A-Z2-7` alphabet, `=` padding).
fn str_to_base32(text: &str) -> String {
    encode_base32(text.as_bytes(), BASE32_STANDARD)
}

/// `"Hé"` → `"H=C3=A9"` (RFC 2045 §6.7; CR/LF become `=0D=0A`; no soft line breaks or trailing-whitespace rule).
fn str_to_quoted_printable(text: &str) -> String {
    escape_bytes(text, '=', is_quoted_printable_literal)
}

// /// `"bücher"` → `"bcher-kva"` (RFC 3492 Bootstring).
// fn str_to_punycode(text: &str) -> String {
//     todo!("{text}")
// }

/// `"H"` → `"⡈"` (one 8-dot cell per UTF-8 byte, `U+2800 + byte`).
fn str_to_braille_patterns(text: &str) -> String {
    join_units(text.bytes(), "", |out, byte| {
        let cell = char::from_u32(0x2800 + u32::from(byte))
            .expect("U+2800..=U+28FF are all assigned braille patterns");
        out.push(cell);
        Ok(())
    })
}

// /// `"e\u{301}"` → `"é"` (canonical composition).
// fn str_to_nfc(text: &str) -> String {
//     todo!("{text}")
// }

/// `"é"` → `"e\u{301}"` (canonical decomposition).
fn str_to_nfd(text: &str) -> String {
    text.nfd().collect()
}

/// `"ﬁ²"` → `"fi2"` (compatibility composition).
fn str_to_nfkc(text: &str) -> String {
    text.nfkc().collect()
}

/// `"ﬁé"` → `"fie\u{301}"` (compatibility decomposition).
fn str_to_nfkd(text: &str) -> String {
    text.nfkd().collect()
}

/// `"e\u{301}👍🏽"` → `"[e\u{301}] [👍🏽]"` (extended grapheme clusters).
fn str_to_graphemes(text: &str) -> String {
    join_units(text.graphemes(true), " ", |out, grapheme| {
        write!(out, "[{grapheme}]")
    })
}

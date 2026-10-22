use hw_pp_core::data::notation::Notation;

#[test]
fn renders_per_unit_notations() {
    // Space (below 64) catches missing zero-padding in binary and octal; `é` adds
    // two bytes above 127. Hex padding is not exercised (no byte below 16).
    for (notation, expected) in [
        (Notation::Binary, "01001000 00100000 11000011 10101001"),
        (Notation::Octal, "110 040 303 251"),
        (Notation::Decimal, "72 32 195 169"),
        (Notation::LowerHex, "48 20 c3 a9"),
        (Notation::UpperHex, "48 20 C3 A9"),
        (Notation::CodePoints, "U+0048 U+0020 U+00E9"),
    ] {
        assert_eq!(notation.render("H é"), expected, "{notation}");
    }
}

#[test]
fn renders_empty_text_as_empty() {
    assert_eq!(Notation::Binary.render(""), "");
}

#[test]
fn renders_base_encodings() {
    // RFC 4648 §10 test vectors cover every padding length.
    for (input, base64, base32) in [
        ("", "", ""),
        ("f", "Zg==", "MY======"),
        ("fo", "Zm8=", "MZXQ===="),
        ("foo", "Zm9v", "MZXW6==="),
        ("foob", "Zm9vYg==", "MZXW6YQ="),
        ("fooba", "Zm9vYmE=", "MZXW6YTB"),
        ("foobar", "Zm9vYmFy", "MZXW6YTBOI======"),
        ("Hé", "SMOp", "JDB2S==="),
    ] {
        assert_eq!(Notation::Base64.render(input), base64, "Base64 {input:?}");
        assert_eq!(Notation::Base32.render(input), base32, "Base32 {input:?}");
    }
}

#[test]
fn renders_base64_url_alphabet() {
    // UTF-8 bytes EF AF BF hit the last two alphabet slots: `+/` vs `-_`.
    assert_eq!(Notation::Base64.render("\u{fbff}"), "76+/");
    assert_eq!(Notation::Base64Url.render("\u{fbff}"), "76-_");
}

#[test]
fn renders_braille_patterns() {
    // Dot n = bit n - 1, so the cell offset is the byte itself.
    for (input, expected) in [("", ""), ("H", "⡈"), ("Hé", "⡈⣃⢩"), ("\0", "⠀")] {
        assert_eq!(Notation::Braille.render(input), expected, "{input:?}");
    }
}

#[test]
fn renders_normalization_forms() {
    // Precomposed é (U+00E9) vs e + combining acute (U+0301); ﬁ ligature and superscript ².
    for (input, nfd, nfkc, nfkd) in [
        ("", "", "", ""),
        (
            "Hello, World!",
            "Hello, World!",
            "Hello, World!",
            "Hello, World!",
        ),
        ("é", "e\u{301}", "é", "e\u{301}"),
        ("e\u{301}", "e\u{301}", "é", "e\u{301}"),
        ("ﬁ²", "ﬁ²", "fi2", "fi2"),
        ("ﬁé", "ﬁe\u{301}", "fié", "fie\u{301}"),
    ] {
        assert_eq!(Notation::Nfd.render(input), nfd, "NFD {input:?}");
        assert_eq!(Notation::Nfkc.render(input), nfkc, "NFKC {input:?}");
        assert_eq!(Notation::Nfkd.render(input), nfkd, "NFKD {input:?}");
    }
}

#[test]
fn renders_escape_encodings() {
    for (input, percent, quoted_printable) in [
        ("Hello, World!", "Hello%2C%20World%21", "Hello, World!"),
        ("a=b", "a%3Db", "a=3Db"),
        ("H é", "H%20%C3%A9", "H =C3=A9"),
        ("\0\x7F", "%00%7F", "=00=7F"),
    ] {
        assert_eq!(
            Notation::PercentEncoding.render(input),
            percent,
            "Percent {input:?}"
        );
        assert_eq!(
            Notation::QuotedPrintable.render(input),
            quoted_printable,
            "QuotedPrintable {input:?}"
        );
    }
}

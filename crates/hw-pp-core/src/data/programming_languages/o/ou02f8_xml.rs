//! Hello-world snippet for `ProgrammingLanguageName::OU02f8Xml`.
//!
//! Imported from leachim6/hello-world (`o/o˸XML` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/o%CB%B8XML>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OU02f8Xml => "o˸XML",
    versions: [OU02f8Xml => "o˸XML"],
    extensions: [],
}

const OU02F8_XML: &str = concat!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<text><![CDATA["#,
    output_for_hello_world!(),
    r"]]></text>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OU02f8Xml,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OU02f8Xml,
            syntax: OU02F8_XML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OU02F8_XML,
                    [
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"?xml version="),
                        (TokenKind::String, r#""1.0""#),
                        (TokenKind::Plain, r" encoding="),
                        (TokenKind::String, r#""UTF-8""#),
                        (TokenKind::Plain, r"?"),
                        (TokenKind::Keyword, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"<text><"),
                        (
                            TokenKind::Plain,
                            concat!(r"![CDATA[", output_for_hello_world!(), r"]]"),
                        ),
                        (TokenKind::Keyword, r"></text>"),
                    ],
                )
            },
        }],
    };

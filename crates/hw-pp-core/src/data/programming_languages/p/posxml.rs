//! Hello-world snippet for `ProgrammingLanguageName::Posxml`.
//!
//! Imported from leachim6/hello-world (`p/PosXML.posxml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PosXML.posxml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Posxml => "PosXML",
    versions: [Posxml => "PosXML"],
    extensions: [Posxml => "posxml"],
}

const POSXML: &str = concat!(
    r#"<display line="0" column="0" message=""#,
    output_for_hello_world!(),
    r#"" />"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Posxml,
        extensions: &[FileExtension::Posxml],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Posxml,
            syntax: POSXML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POSXML,
                    [
                        (TokenKind::Keyword, r"<display"),
                        (TokenKind::Plain, r" line="),
                        (TokenKind::String, r#""0""#),
                        (TokenKind::Plain, r" column="),
                        (TokenKind::String, r#""0""#),
                        (TokenKind::Plain, r" message="),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" /"),
                        (TokenKind::Keyword, r">"),
                    ],
                )
            },
        }],
    };

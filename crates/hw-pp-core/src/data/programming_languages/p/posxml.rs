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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Posxml => "PosXML",
    versions: [Posxml => "PosXML"],
    extensions: [Posxml => "posxml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Posxml,
        extensions: &[FileExtension::Posxml],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Posxml,
            syntax: concat!(
                r#"<display line="0" column="0" message=""#,
                output_for_hello_world!(),
                r#"" />"#
            ),
            tokenized_syntax: &[
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
        }],
    };

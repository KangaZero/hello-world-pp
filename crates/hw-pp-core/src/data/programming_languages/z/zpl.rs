//! Hello-world snippet for `ProgrammingLanguageName::Zpl`.
//!
//! Imported from leachim6/hello-world (`z/ZPL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/ZPL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zpl => "ZPL",
    versions: [Zpl => "ZPL"],
    extensions: [],
}

const ZPL: &str = concat!(r"^XA^FD", output_for_hello_world!(), r"^XZ~PS");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zpl,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zpl,
            syntax: ZPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZPL,
                    [(
                        TokenKind::Plain,
                        concat!(r"^XA^FD", output_for_hello_world!(), r"^XZ~PS"),
                    )],
                )
            },
        }],
    };

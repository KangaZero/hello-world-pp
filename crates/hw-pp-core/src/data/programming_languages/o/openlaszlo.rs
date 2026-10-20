//! Hello-world snippet for `ProgrammingLanguageName::Openlaszlo`.
//!
//! Imported from leachim6/hello-world (`o/OpenLaszlo.lzx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/OpenLaszlo.lzx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Openlaszlo => "OpenLaszlo",
    versions: [Openlaszlo => "OpenLaszlo"],
    extensions: [Lzx => "lzx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Openlaszlo,
        extensions: &[FileExtension::Lzx],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Openlaszlo,
            syntax: concat!(
                r"<canvas>
    <text>",
                output_for_hello_world!(),
                r"</text>
</canvas>"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<canvas>"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"<text>"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Keyword, r"</text>"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"</canvas>"),
            ],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Leszek`.
//!
//! Imported from leachim6/hello-world (`l/Leszek.leszek` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Leszek.leszek>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Leszek => "Leszek",
    versions: [Leszek => "Leszek"],
    extensions: [Leszek => "leszek"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Leszek,
        extensions: &[FileExtension::Leszek],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Leszek,
            syntax: concat!(r"\O\C\.1.", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"\O\C\.1.", output_for_hello_world!()),
            )],
        }],
    };

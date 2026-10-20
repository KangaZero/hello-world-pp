//! Hello-world snippet for `ProgrammingLanguageName::PleasePorigeHot`.
//!
//! Imported from leachim6/hello-world (`p/Please Porige Hot.pph` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Please%20Porige%20Hot.pph>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PleasePorigeHot => "Please Porige Hot",
    versions: [PleasePorigeHot => "Please Porige Hot"],
    extensions: [Pph => "pph"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PleasePorigeHot,
        extensions: &[FileExtension::Pph],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PleasePorigeHot,
            syntax: concat!(r"1 Please porige hot or cold ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"1 Please porige hot or cold ", output_for_hello_world!()),
            )],
        }],
    };

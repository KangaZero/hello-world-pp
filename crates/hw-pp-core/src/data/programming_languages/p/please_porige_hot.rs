//! Hello-world snippet for `ProgrammingLanguageName::PleasePorigeHot`.
//!
//! Imported from leachim6/hello-world (`p/Please Porige Hot.pph` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PleasePorigeHot,
            syntax: concat!(r"1 Please porige hot or cold ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"1 Please porige hot or cold ", output_for_hello_world!()),
            )],
        }],
    };

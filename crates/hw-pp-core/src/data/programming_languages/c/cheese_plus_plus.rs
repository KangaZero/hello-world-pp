//! Hello-world snippet for `ProgrammingLanguageName::CheesePlusPlus`.
//!
//! Imported from leachim6/hello-world (`c/Cheese++.cheese` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cheese%2B%2B.cheese>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CheesePlusPlus => "Cheese++",
    versions: [CheesePlusPlus => "Cheese++"],
    extensions: [Cheese => "cheese"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CheesePlusPlus,
        extensions: &[FileExtension::Cheese],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CheesePlusPlus,
            syntax: concat!(
                r"Cheese
    Wensleydale(Swiss",
                output_for_hello_world!(),
                r"Swiss)Brie
NoCheese"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"Cheese
    Wensleydale(Swiss",
                    output_for_hello_world!(),
                    r"Swiss)Brie
NoCheese"
                ),
            )],
        }],
    };

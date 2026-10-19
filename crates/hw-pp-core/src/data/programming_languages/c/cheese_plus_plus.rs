//! Hello-world snippet for `ProgrammingLanguageName::CheesePlusPlus`.
//!
//! Imported from leachim6/hello-world (`c/Cheese++.cheese` at `a152253`),
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
    language: CheesePlusPlus => "Cheese++",
    versions: [CheesePlusPlus => "Cheese++"],
    extensions: [Cheese => "cheese"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CheesePlusPlus,
        extensions: &[FileExtension::Cheese],
        categories: &[],
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

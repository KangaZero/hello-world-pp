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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: CheesePlusPlus => "Cheese++",
    versions: [CheesePlusPlus => "Cheese++"],
    extensions: [Cheese => "cheese"],
}

const CHEESE_PLUS_PLUS: &str = concat!(
    r"Cheese
    Wensleydale(Swiss",
    output_for_hello_world!(),
    r"Swiss)Brie
NoCheese"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CheesePlusPlus,
        extensions: &[FileExtension::Cheese],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CheesePlusPlus,
            syntax: CHEESE_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHEESE_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Cheese
    Wensleydale(Swiss",
                            output_for_hello_world!(),
                            r"Swiss)Brie
NoCheese"
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Dataflex`.
//!
//! Imported from leachim6/hello-world (`d/DataFlex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DataFlex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dataflex => "DataFlex",
    versions: [Dataflex => "DataFlex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dataflex,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dataflex,
            syntax: concat!(
                r"/tela

",
                output_for_hello_world!(),
                r"

/*

clearscreen

page tela"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"/tela

",
                    output_for_hello_world!(),
                    r"

/*

clearscreen

page tela"
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Dataflex`.
//!
//! Imported from leachim6/hello-world (`d/DataFlex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DataFlex>
//! - <https://www.wikidata.org/wiki/Q2391203> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Structured,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dataflex => "DataFlex",
    versions: [Dataflex => "DataFlex"],
    extensions: [],
}

const DATAFLEX: &str = concat!(
    r"/tela

",
    output_for_hello_world!(),
    r"

/*

clearscreen

page tela"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dataflex,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Structured, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dataflex,
            syntax: DATAFLEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DATAFLEX,
                    [(
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
                )
            },
        }],
    };

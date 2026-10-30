//! Hello-world snippet for `ProgrammingLanguageName::BinaryLambdaCalculus`.
//!
//! Imported from the Hello World Collection (`files/Binary&nbsp;Lambda&nbsp;Calculus.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Binary%26nbsp%3BLambda%26nbsp%3BCalculus.htm>
//! - <http://helloworldcollection.de/#Binary%C2%A0Lambda%C2%A0Calculus>
//! - <https://esolangs.org/wiki/Binary_lambda_calculus> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: BinaryLambdaCalculus => "Binary Lambda Calculus",
    versions: [BinaryLambdaCalculus => "Binary Lambda Calculus"],
    extensions: [],
}

const BINARY_LAMBDA_CALCULUS: &str = concat!(r"!", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BinaryLambdaCalculus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BinaryLambdaCalculus,
            syntax: BINARY_LAMBDA_CALCULUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BINARY_LAMBDA_CALCULUS,
                    [(TokenKind::Plain, concat!(r"!", output_for_hello_world!()))],
                )
            },
        }],
    };

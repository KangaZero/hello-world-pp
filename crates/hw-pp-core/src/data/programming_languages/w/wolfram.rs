//! Hello-world snippet for `ProgrammingLanguageName::Wolfram`.
//!
//! Imported from the Hello World Collection (`files/Wolfram.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Wolfram.htm>
//! - <http://helloworldcollection.de/#Wolfram>
//! - <https://www.wikidata.org/wiki/Q15241057> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Array, Declarative, Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Wolfram => "Wolfram",
    versions: [Wolfram => "Wolfram"],
    extensions: [],
}

const WOLFRAM: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in Wolfram Language *)

CloudDeploy[""#,
    output_for_hello_world!(),
    r#""]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wolfram,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, Declarative, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wolfram,
            syntax: WOLFRAM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WOLFRAM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"(* ",
                            output_for_hello_world!(),
                            r#" in Wolfram Language *)

CloudDeploy[""#,
                            output_for_hello_world!(),
                            r#""]"#
                        ),
                    )],
                )
            },
        }],
    };

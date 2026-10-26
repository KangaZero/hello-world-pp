//! Hello-world snippet for `ProgrammingLanguageName::Informix4gl`.
//!
//! Imported from the Hello World Collection (`files/Informix&nbsp;4GL` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Informix%26nbsp%3B4GL>
//! - <http://helloworldcollection.de/#Informix%C2%A04GL>
//! - <https://en.wikipedia.org/wiki/Informix-4GL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, Imperative, Procedural, Structured,
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
    language: Informix4gl => "Informix 4GL",
    versions: [Informix4gl => "Informix 4GL"],
    extensions: [],
}

const INFORMIX4GL: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Informix 4GL

MAIN

  DISPLAY ""#,
    output_for_hello_world!(),
    r#""

END MAIN"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Informix4gl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Informix4gl,
            syntax: INFORMIX4GL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    INFORMIX4GL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in Informix 4GL

MAIN

  DISPLAY ""#,
                            output_for_hello_world!(),
                            r#""

END MAIN"#
                        ),
                    )],
                )
            },
        }],
    };

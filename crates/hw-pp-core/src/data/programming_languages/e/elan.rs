//! Hello-world snippet for `ProgrammingLanguageName::Elan`.
//!
//! Imported from the Hello World Collection (`files/Elan` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Elan>
//! - <http://helloworldcollection.de/#Elan>
//! - <https://en.wikipedia.org/wiki/ELAN_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Procedural, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Algol,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Elan => "Elan",
    versions: [Elan => "Elan"],
    extensions: [],
}

const ELAN: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in ELAN *)

putline (""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elan,
        extensions: &[],
        categories: &[],
        families: &[Algol],
        paradigms: &[Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elan,
            syntax: ELAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELAN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"(* ",
                            output_for_hello_world!(),
                            r#" in ELAN *)

putline (""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };

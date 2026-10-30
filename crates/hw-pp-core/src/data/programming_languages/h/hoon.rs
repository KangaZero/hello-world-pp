//! Hello-world snippet for `ProgrammingLanguageName::Hoon`.
//!
//! Imported from the Hello World Collection (`files/Hoon.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Hoon.htm>
//! - <http://helloworldcollection.de/#Hoon>
//! - <https://esolangs.org/wiki/Hoon> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Hoon => "Hoon",
    versions: [Hoon => "Hoon"],
    extensions: [],
}

const HOON: &str = concat!(
    r":: ",
    output_for_hello_world!(),
    r#" in Hoon

!:(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hoon,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hoon,
            syntax: HOON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HOON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r":: ",
                            output_for_hello_world!(),
                            r#" in Hoon

!:(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };

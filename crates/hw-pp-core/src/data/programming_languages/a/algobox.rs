//! Hello-world snippet for `ProgrammingLanguageName::Algobox`.
//!
//! Imported from leachim6/hello-world (PR #1737 `a/Algobox.alg`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1737>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Educational, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Algobox => "Algobox",
    versions: [Algobox => "Algobox"],
    extensions: [],
}

const ALGOBOX: &str = concat!(
    r#"FONCTIONS_UTILISEES
VARIABLES
DEBUT_ALGORITHME
  AFFICHER ""#,
    output_for_hello_world!(),
    r#""
FIN_ALGORITHME"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algobox,
        extensions: &[],
        categories: &[Educational],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algobox,
            syntax: ALGOBOX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALGOBOX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"FONCTIONS_UTILISEES
VARIABLES
DEBUT_ALGORITHME
  AFFICHER ""#,
                            output_for_hello_world!(),
                            r#""
FIN_ALGORITHME"#
                        ),
                    )],
                )
            },
        }],
    };

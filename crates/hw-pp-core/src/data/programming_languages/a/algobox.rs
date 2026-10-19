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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Algobox => "Algobox",
    versions: [Algobox => "Algobox"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algobox,
        extensions: &[],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algobox,
            syntax: concat!(
                r#"FONCTIONS_UTILISEES
VARIABLES
DEBUT_ALGORITHME
  AFFICHER ""#,
                output_for_hello_world!(),
                r#""
FIN_ALGORITHME"#
            ),
            tokenized_syntax: &[(
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
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Visualg`.
//!
//! Imported from leachim6/hello-world (`v/VisuAlg.alg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VisuAlg.alg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Educational, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Visualg => "VisuAlg",
    versions: [Visualg => "VisuAlg"],
    extensions: [],
}

const VISUALG: &str = concat!(
    r#"Algoritmo " "
Inicio
escreva(""#,
    output_for_hello_world!(),
    r#"")
Fimalgoritmo"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Visualg,
        extensions: &[],
        categories: &[Educational],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Visualg,
            syntax: VISUALG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VISUALG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Algoritmo " "
Inicio
escreva(""#,
                            output_for_hello_world!(),
                            r#"")
Fimalgoritmo"#
                        ),
                    )],
                )
            },
        }],
    };

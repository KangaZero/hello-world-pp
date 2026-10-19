//! Hello-world snippet for `ProgrammingLanguageName::Visualg`.
//!
//! Imported from leachim6/hello-world (`v/VisuAlg.alg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Visualg => "VisuAlg",
    versions: [Visualg => "VisuAlg"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Visualg,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Visualg,
            syntax: concat!(
                r#"Algoritmo " "
Inicio
escreva(""#,
                output_for_hello_world!(),
                r#"")
Fimalgoritmo"#
            ),
            tokenized_syntax: &[(
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
        }],
    };

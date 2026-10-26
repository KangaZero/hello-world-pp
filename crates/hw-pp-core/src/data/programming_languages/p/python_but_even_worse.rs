//! Hello-world snippet for `ProgrammingLanguageName::PythonButEvenWorse`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Python_but_even_worse>
//! - <https://esolangs.org/wiki/Python_but_even_worse>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PythonButEvenWorse => "Python but even worse",
    versions: [PythonButEvenWorse => "Python but even worse"],
    extensions: [],
}

const PYTHON_BUT_EVEN_WORSE: &str = concat!(
    r#"print(""#,
    output_for_hello_world!(),
    r#"")
1/0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PythonButEvenWorse,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PythonButEvenWorse,
            syntax: PYTHON_BUT_EVEN_WORSE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PYTHON_BUT_EVEN_WORSE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"print(""#,
                            output_for_hello_world!(),
                            r#"")
1/0"#
                        ),
                    )],
                )
            },
        }],
    };

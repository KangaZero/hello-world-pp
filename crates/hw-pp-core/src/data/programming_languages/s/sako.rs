//! Hello-world snippet for `ProgrammingLanguageName::Sako`.
//!
//! Imported from leachim6/hello-world (PR #1542 `s/SAKO.sako`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1542>
//! - <https://en.wikipedia.org/wiki/SAKO_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Array, Imperative, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sako => "SAKO",
    versions: [Sako => "SAKO"],
    extensions: [Sako => "sako"],
}

const SAKO: &str = concat!(
    r"TEKST:
",
    output_for_hello_world!(),
    r"
STOP NASTEPNY
KONIEC"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sako,
        extensions: &[FileExtension::Sako],
        categories: &[Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sako,
            syntax: SAKO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SAKO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"TEKST:
",
                            output_for_hello_world!(),
                            r"
STOP NASTEPNY
KONIEC"
                        ),
                    )],
                )
            },
        }],
    };

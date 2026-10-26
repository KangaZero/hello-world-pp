//! Hello-world snippet for `ProgrammingLanguageName::Orion`.
//!
//! Imported from leachim6/hello-world (`o/Orion.orn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Orion.orn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Orion => "Orion",
    versions: [Orion => "Orion"],
    extensions: [Orn => "orn"],
}

const ORION: &str = concat!(
    r#"(def 'impure main
     (λ () (putStrLn ""#,
    output_for_hello_world!(),
    r#"")))"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Orion,
        extensions: &[FileExtension::Orn],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Orion,
            syntax: ORION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ORION,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"(def 'impure main
     (λ () (putStrLn ""#,
                            output_for_hello_world!(),
                            r#"")))"#
                        ),
                    )],
                )
            },
        }],
    };

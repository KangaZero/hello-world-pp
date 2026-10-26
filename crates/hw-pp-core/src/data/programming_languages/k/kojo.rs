//! Hello-world snippet for `ProgrammingLanguageName::Kojo`.
//!
//! Imported from leachim6/hello-world (`k/Kojo.kojo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kojo.kojo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Kojo => "Kojo",
    versions: [Kojo => "Kojo"],
    extensions: [Kojo => "kojo"],
}

const KOJO: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kojo,
        extensions: &[FileExtension::Kojo],
        categories: &[Educational],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kojo,
            syntax: KOJO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KOJO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

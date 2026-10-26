//! Hello-world snippet for `ProgrammingLanguageName::Uniface`.
//!
//! Imported from leachim6/hello-world (`u/Uniface.uniface` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Uniface.uniface>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Uniface => "Uniface",
    versions: [Uniface => "Uniface"],
    extensions: [Uniface => "uniface"],
}

const UNIFACE: &str = concat!(r#"message ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Uniface,
        extensions: &[FileExtension::Uniface],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Uniface,
            syntax: UNIFACE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNIFACE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"message ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::ThrowThisToThat`.
//!
//! Imported from leachim6/hello-world (`t/THROW THIS TO THAT` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/THROW%20THIS%20TO%20THAT>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ThrowThisToThat => "THROW THIS TO THAT",
    versions: [ThrowThisToThat => "THROW THIS TO THAT"],
    extensions: [],
}

const THROW_THIS_TO_THAT: &str = concat!(r#"   THROW ""#, output_for_hello_world!(), r#"" TO OUT"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ThrowThisToThat,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ThrowThisToThat,
            syntax: THROW_THIS_TO_THAT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    THROW_THIS_TO_THAT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"   THROW ""#, output_for_hello_world!(), r#"" TO OUT"#),
                    )],
                )
            },
        }],
    };

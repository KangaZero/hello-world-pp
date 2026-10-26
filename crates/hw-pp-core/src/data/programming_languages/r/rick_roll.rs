//! Hello-world snippet for `ProgrammingLanguageName::RickRoll`.
//!
//! Imported from leachim6/hello-world (`r/Rick Roll.rickroll` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rick%20Roll.rickroll>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: RickRoll => "Rick Roll",
    versions: [RickRoll => "Rick Roll"],
    extensions: [Rickroll => "rickroll"],
}

const RICK_ROLL: &str = concat!(
    r#"take me to ur heart
    give msg up ""#,
    output_for_hello_world!(),
    r#""
    i just wanna tell u how im feeling msg
say goodbye"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RickRoll,
        extensions: &[FileExtension::Rickroll],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RickRoll,
            syntax: RICK_ROLL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RICK_ROLL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"take me to ur heart
    give msg up ""#,
                            output_for_hello_world!(),
                            r#""
    i just wanna tell u how im feeling msg
say goodbye"#
                        ),
                    )],
                )
            },
        }],
    };

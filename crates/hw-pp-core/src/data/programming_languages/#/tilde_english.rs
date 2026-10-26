//! Hello-world snippet for `ProgrammingLanguageName::TildeEnglish`.
//!
//! Imported from leachim6/hello-world (`#/~English` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/~English>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TildeEnglish => "~English",
    versions: [TildeEnglish => "~English"],
    extensions: [],
}

const TILDE_ENGLISH: &str = concat!(
    r#"Display ""#,
    output_for_hello_world!(),
    r#"" and a newline.
Stop the program."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TildeEnglish,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TildeEnglish,
            syntax: TILDE_ENGLISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TILDE_ENGLISH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Display ""#,
                            output_for_hello_world!(),
                            r#"" and a newline.
Stop the program."#
                        ),
                    )],
                )
            },
        }],
    };

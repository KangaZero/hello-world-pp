//! Hello-world snippet for `ProgrammingLanguageName::Cantonese`.
//!
//! Imported from leachim6/hello-world (`c/Cantonese.cantonese` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cantonese.cantonese>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cantonese => "Cantonese",
    versions: [Cantonese => "Cantonese"],
    extensions: [Cantonese => "cantonese"],
}

const CANTONESE: &str = concat!(
    r#"畀我睇下 " "#,
    output_for_hello_world!(),
    r#" " 点样先?
收工"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cantonese,
        extensions: &[FileExtension::Cantonese],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cantonese,
            syntax: CANTONESE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CANTONESE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"畀我睇下 " "#,
                            output_for_hello_world!(),
                            r#" " 点样先?
收工"#
                        ),
                    )],
                )
            },
        }],
    };

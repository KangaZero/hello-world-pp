//! Hello-world snippet for `ProgrammingLanguageName::Greeeeeeeeentext`.
//!
//! Imported from leachim6/hello-world (`g/GreeeeeeeeenText` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GreeeeeeeeenText>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Greeeeeeeeentext => "GreeeeeeeeenText",
    versions: [Greeeeeeeeentext => "GreeeeeeeeenText"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Greeeeeeeeentext,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Greeeeeeeeentext,
            syntax: concat!(
                r">be me
>says ",
                output_for_hello_world!(),
                r"
>*gets output*"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r">be me
>says ",
                    output_for_hello_world!(),
                    r"
>*gets output*"
                ),
            )],
        }],
    };

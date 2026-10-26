//! Hello-world snippet for `ProgrammingLanguageName::Aeolbonn`.
//!
//! Imported from leachim6/hello-world (`a/Aeolbonn.aeo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Aeolbonn.aeo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aeolbonn => "Aeolbonn",
    versions: [Aeolbonn => "Aeolbonn"],
    extensions: [Aeo => "aeo"],
}

const AEOLBONN: &str = concat!(
    r":",
    output_for_hello_world!(),
    r"
:"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aeolbonn,
        extensions: &[FileExtension::Aeo],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aeolbonn,
            syntax: AEOLBONN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AEOLBONN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r":",
                            output_for_hello_world!(),
                            r"
:"
                        ),
                    )],
                )
            },
        }],
    };

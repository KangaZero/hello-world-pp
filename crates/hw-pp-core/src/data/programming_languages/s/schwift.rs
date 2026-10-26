//! Hello-world snippet for `ProgrammingLanguageName::Schwift`.
//!
//! Imported from leachim6/hello-world (`s/SCHWift.y` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SCHWift.y>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Schwift => "SCHWift",
    versions: [Schwift => "SCHWift"],
    extensions: [Y => "y"],
}

const SCHWIFT: &str = concat!(
    r#"show me what you got ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Schwift,
        extensions: &[FileExtension::Y],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Schwift,
            syntax: SCHWIFT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SCHWIFT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"show me what you got ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

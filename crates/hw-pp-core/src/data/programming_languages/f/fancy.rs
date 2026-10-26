//! Hello-world snippet for `ProgrammingLanguageName::Fancy`.
//!
//! Imported from leachim6/hello-world (`f/Fancy.fy` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fancy.fy>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fancy => "Fancy",
    versions: [Fancy => "Fancy"],
    extensions: [Fy => "fy"],
}

const FANCY: &str = concat!(r#"""#, output_for_hello_world!(), r#"" println "#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fancy,
        extensions: &[FileExtension::Fy],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fancy,
            syntax: FANCY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FANCY,
                    [
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Plain, r" "),
                    ],
                )
            },
        }],
    };

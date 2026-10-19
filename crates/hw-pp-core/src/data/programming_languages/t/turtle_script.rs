//! Hello-world snippet for `ProgrammingLanguageName::TurtleScript`.
//!
//! Imported from leachim6/hello-world (`t/Turtle Script.turtle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Turtle%20Script.turtle>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: TurtleScript => "Turtle Script",
    versions: [TurtleScript => "Turtle Script"],
    extensions: [Turtle => "turtle"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TurtleScript,
        extensions: &[FileExtension::Turtle],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TurtleScript,
            syntax: concat!(
                r#"kturtle-script-v1.0
@(print) ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"kturtle-script-v1.0
@(print) ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

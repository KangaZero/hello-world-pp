//! Hello-world snippet for `ProgrammingLanguageName::RobotFramework`.
//!
//! Imported from leachim6/hello-world (`r/Robot Framework.robot` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Robot%20Framework.robot>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: RobotFramework => "Robot Framework",
    versions: [RobotFramework => "Robot Framework"],
    extensions: [Robot => "robot"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RobotFramework,
        extensions: &[FileExtension::Robot],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RobotFramework,
            syntax: concat!(
                r"*** Task ***
Display Greeting
    Log To Console    ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"*** Task ***
Display Greeting
    Log To Console    ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };

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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: RobotFramework => "Robot Framework",
    versions: [RobotFramework => "Robot Framework"],
    extensions: [Robot => "robot"],
}

const ROBOT_FRAMEWORK: &str = concat!(
    r"*** Task ***
Display Greeting
    Log To Console    ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RobotFramework,
        extensions: &[FileExtension::Robot],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RobotFramework,
            syntax: ROBOT_FRAMEWORK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ROBOT_FRAMEWORK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"*** Task ***
Display Greeting
    Log To Console    ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };

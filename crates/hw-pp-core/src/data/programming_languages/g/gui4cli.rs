//! Hello-world snippet for `ProgrammingLanguageName::Gui4cli`.
//!
//! Imported from leachim6/hello-world (`g/Gui4Cli` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gui4Cli>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gui4cli => "Gui4Cli",
    versions: [Gui4cli => "Gui4Cli"],
    extensions: [],
}

const GUI4CLI: &str = concat!(
    r#"G4C MyGui
Window 10 10 200 300 "My window"

xOnLoad
   GuiOpen MyGui

xButton 10 10 100 20 ""#,
    output_for_hello_world!(),
    r#""
   GuiQuit MyGui"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gui4cli,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gui4cli,
            syntax: GUI4CLI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GUI4CLI,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"G4C MyGui
Window 10 10 200 300 "My window"

xOnLoad
   GuiOpen MyGui

xButton 10 10 100 20 ""#,
                            output_for_hello_world!(),
                            r#""
   GuiQuit MyGui"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::VisualBasicScript`.
//!
//! Imported from leachim6/hello-world (`v/Visual Basic Script.vbs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Visual%20Basic%20Script.vbs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VisualBasicScript => "Visual Basic Script",
    versions: [VisualBasicScript => "Visual Basic Script"],
    extensions: [Vbs => "vbs"],
}

const VISUAL_BASIC_SCRIPT: &str = concat!(r#"MsgBox ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasicScript,
        extensions: &[FileExtension::Vbs],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualBasicScript,
            syntax: VISUAL_BASIC_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VISUAL_BASIC_SCRIPT,
                    [
                        (TokenKind::Keyword, r"MsgBox"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };

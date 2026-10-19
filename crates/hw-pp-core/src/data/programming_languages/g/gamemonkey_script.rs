//! Hello-world snippet for `ProgrammingLanguageName::GamemonkeyScript`.
//!
//! Imported from leachim6/hello-world (PR #1824 `g/gamemonkeyscript.gm`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1824>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GamemonkeyScript => "GameMonkey Script",
    versions: [GamemonkeyScript => "GameMonkey Script"],
    extensions: [Gm => "gm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GamemonkeyScript,
        extensions: &[FileExtension::Gm],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GamemonkeyScript,
            syntax: concat!(
                r#"global HelloWorld = function()
{
    print(""#,
                output_for_hello_world!(),
                r#"");
};

// Execute the function
HelloWorld();"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"global HelloWorld = function()
{
    print(""#,
                    output_for_hello_world!(),
                    r#"");
};

// Execute the function
HelloWorld();"#
                ),
            )],
        }],
    };

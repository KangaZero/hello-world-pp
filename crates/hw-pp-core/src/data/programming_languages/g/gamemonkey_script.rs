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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GamemonkeyScript => "GameMonkey Script",
    versions: [GamemonkeyScript => "GameMonkey Script"],
    extensions: [Gm => "gm"],
}

const GAMEMONKEY_SCRIPT: &str = concat!(
    r#"global HelloWorld = function()
{
    print(""#,
    output_for_hello_world!(),
    r#"");
};

// Execute the function
HelloWorld();"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GamemonkeyScript,
        extensions: &[FileExtension::Gm],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GamemonkeyScript,
            syntax: GAMEMONKEY_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GAMEMONKEY_SCRIPT,
                    [
                        (TokenKind::Plain, r"global HelloWorld = "),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"};"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Comment, r"// Execute the function"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"HelloWorld"),
                        (TokenKind::Punctuation, r"();"),
                    ],
                )
            },
        }],
    };

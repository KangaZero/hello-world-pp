//! Hello-world snippet for `ProgrammingLanguageName::Nwscript`.
//!
//! Imported from leachim6/hello-world (`n/NWScript.nss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/NWScript.nss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Nwscript => "NWScript",
    versions: [Nwscript => "NWScript"],
    extensions: [Nss => "nss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nwscript,
        extensions: &[FileExtension::Nss],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nwscript,
            syntax: concat!(
                r#"// Place in the OnClientEnter event of the module's properties
{
    SendMessageToPC(GetEnteringObject(), ""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    r"// Place in the OnClientEnter event of the module's properties",
                ),
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
                (TokenKind::Function, r"SendMessageToPC"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"GetEnteringObject"),
                (TokenKind::Punctuation, r"(),"),
                (TokenKind::Plain, r" "),
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
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };

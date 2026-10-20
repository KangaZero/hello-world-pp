//! Hello-world snippet for `ProgrammingLanguageName::Autohotkey`.
//!
//! Imported from leachim6/hello-world (`a/AutoHotKey.ahk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AutoHotKey.ahk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Autohotkey => "AutoHotKey",
    versions: [Autohotkey => "AutoHotKey"],
    extensions: [Ahk => "ahk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Autohotkey,
        extensions: &[FileExtension::Ahk],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Autohotkey,
            syntax: concat!(r"MsgBox, ", output_for_hello_world!()),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"MsgBox"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
            ],
        }],
    };

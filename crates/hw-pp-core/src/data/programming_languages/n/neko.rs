//! Hello-world snippet for `ProgrammingLanguageName::Neko`.
//!
//! Imported from leachim6/hello-world (`n/Neko.neko` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Neko.neko>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Neko => "Neko",
    versions: [Neko => "Neko"],
    extensions: [Neko => "neko"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Neko,
        extensions: &[FileExtension::Neko],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Neko,
            syntax: concat!(r#"$print(""#, output_for_hello_world!(), r#"\n");"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"$print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };

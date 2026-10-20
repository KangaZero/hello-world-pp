//! Hello-world snippet for `ProgrammingLanguageName::Io`.
//!
//! Imported from leachim6/hello-world (`i/Io.Io` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Io.Io>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Io => "Io",
    versions: [Io => "Io"],
    extensions: [Io => "Io"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Io,
        extensions: &[FileExtension::Io],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Io,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"\n" print"#),
            tokenized_syntax: &[
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"print"),
            ],
        }],
    };

//! Hello-world snippets for `ProgrammingLanguageName::FSharp`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/fsharp/get-started/get-started-command-line>
//! - <https://learn.microsoft.com/en-us/dotnet/fsharp/whats-new/fsharp-8>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: FSharp => "F#",
    versions: [FSharp8 => "F# 8"],
    extensions: [Fs => "fs", Fsx => "fsx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &[FileExtension::Fs, FileExtension::Fsx],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FSharp8,
            syntax: concat!(r#"printfn ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "printfn"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::FSharp`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/fsharp/get-started/get-started-command-line>
//! - <https://learn.microsoft.com/en-us/dotnet/fsharp/whats-new/fsharp-8>
//! - <https://www.wikidata.org/wiki/Q648619> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Functional, Imperative, Metaprogramming, ObjectOriented, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{DotNet, Ml, OCaml},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: FSharp => "F#",
    versions: [FSharp8 => "F# 8"],
    extensions: [Fs => "fs", Fsx => "fsx"],
}

const F_SHARP8: &str = concat!(r#"printfn ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &[FileExtension::Fs, FileExtension::Fsx],
        categories: &[Compiled],
        families: &[DotNet, Ml, OCaml],
        paradigms: &[
            Imperative,
            ObjectOriented,
            Functional,
            Concurrent,
            Reflective,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FSharp8,
            syntax: F_SHARP8,
            tokenized_syntax: &const {
                checked_token_lengths(
                    F_SHARP8,
                    [
                        (TokenKind::Function, "printfn"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };

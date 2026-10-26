//! Hello-world snippets for `ProgrammingLanguageName::CSharp`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/tour-of-csharp/tutorials/hello-world>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/program-structure/top-level-statements>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CSharp.cs>
//! - <https://www.wikidata.org/wiki/Q2370> (paradigms)
//!
//! The `C# 9` version is imported from leachim6/hello-world,
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::ProgrammingParadigm::{
    EventDriven, Functional, Generic, Imperative, ObjectOriented, Reflective, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{C, DotNet},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CSharp => "C#",
    versions: [CSharp10 => "C# 10", CSharp9 => "C# 9"],
    extensions: [Cs => "cs"],
}

const C_SHARP10: &str = concat!(
    r#"Console.WriteLine(""#,
    output_for_hello_world!(),
    r#"");"#
);
const C_SHARP9: &str = concat!(
    r#"System.Console.WriteLine(""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSharp,
        extensions: &[FileExtension::Cs],
        categories: &[Compiled],
        families: &[C, DotNet],
        paradigms: &[
            Imperative,
            Structured,
            ObjectOriented,
            Functional,
            Reflective,
            Generic,
            EventDriven,
        ],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CSharp10,
                syntax: C_SHARP10,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        C_SHARP10,
                        [
                            (TokenKind::Plain, "Console"),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Function, "WriteLine"),
                            (TokenKind::Punctuation, "("),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                            (TokenKind::Punctuation, ");"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CSharp9,
                syntax: C_SHARP9,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        C_SHARP9,
                        [
                            (TokenKind::Plain, r"System"),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Plain, r"Console"),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Function, r"WriteLine"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r");"),
                        ],
                    )
                },
            },
        ],
    };

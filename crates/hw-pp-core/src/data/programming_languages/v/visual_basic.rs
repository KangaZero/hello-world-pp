//! Hello-world snippets for `ProgrammingLanguageName::VisualBasic`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/getting-started/>
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/programming-guide/program-structure/structure-of-a-visual-basic-program>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Visual%26nbsp%3BBasic%26nbsp%3B6.txt>
//! - <http://helloworldcollection.de/#Visual%C2%A0Basic%C2%A06>
//! - <https://www.wikidata.org/wiki/Q50077> (paradigms)
//!
//! The `Visual Basic 6` version is imported from the Hello World Collection, MIT licensed; see
//! `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::ProgrammingParadigm::{
    EventDriven, Functional, Generic, Imperative, ObjectOriented, Reflective, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Basic, DotNet},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: VisualBasic => "Visual Basic",
    versions: [VbNet => "VB.NET", VisualBasic6 => "Visual Basic 6"],
    extensions: [Vb => "vb"],
}

const VB_NET: &str = concat!(
    r#"Module Program
    Sub Main()
        Console.WriteLine(""#,
    output_for_hello_world!(),
    r#"")
    End Sub
End Module"#
);
const VISUAL_BASIC6: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in Visual Basic 6

Private Sub Form_Load()
Print ""#,
    output_for_hello_world!(),
    r#""
End Sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &[FileExtension::Vb],
        categories: &[Compiled],
        families: &[Basic, DotNet],
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
                label: ProgrammingLanguageDialect::VbNet,
                syntax: VB_NET,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        VB_NET,
                        [
                            (TokenKind::Keyword, "Module"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Type, "Program"),
                            (TokenKind::Plain, "\n    "),
                            (TokenKind::Keyword, "Sub"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Function, "Main"),
                            (TokenKind::Punctuation, "()"),
                            (TokenKind::Plain, "\n        "),
                            (TokenKind::Type, "Console"),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Function, "WriteLine"),
                            (TokenKind::Punctuation, "("),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                            (TokenKind::Punctuation, ")"),
                            (TokenKind::Plain, "\n    "),
                            (TokenKind::Keyword, "End"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Keyword, "Sub"),
                            (TokenKind::Plain, "\n"),
                            (TokenKind::Keyword, "End"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Keyword, "Module"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::VisualBasic6,
                syntax: VISUAL_BASIC6,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        VISUAL_BASIC6,
                        [
                            (
                                TokenKind::Comment,
                                concat!("' ", output_for_hello_world!(), " in Visual Basic 6"),
                            ),
                            (TokenKind::Plain, "\n\n"),
                            (TokenKind::Keyword, "Private"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Keyword, "Sub"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Function, "Form_Load"),
                            (TokenKind::Punctuation, "()"),
                            (TokenKind::Plain, "\n"),
                            (TokenKind::Keyword, "Print"),
                            (TokenKind::Plain, " "),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                            (TokenKind::Plain, "\n"),
                            (TokenKind::Keyword, "End"),
                            (TokenKind::Plain, " "),
                            (TokenKind::Keyword, "Sub"),
                        ],
                    )
                },
            },
        ],
    };

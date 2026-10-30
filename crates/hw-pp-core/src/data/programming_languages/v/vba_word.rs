//! Hello-world snippet for `ProgrammingLanguageName::VbaWord`.
//!
//! Imported from the Hello World Collection (`files/VBA&nbsp;(Word).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VBA%26nbsp%3B%28Word%29.htm>
//! - <http://helloworldcollection.de/#VBA%C2%A0(Word)>
//! - <https://en.wikipedia.org/wiki/Visual_Basic_for_Applications> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::EventDriven;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: VbaWord => "VBA (Word)",
    versions: [VbaWord => "VBA (Word)"],
    extensions: [],
}

const VBA_WORD: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in Visual Basic for Applications, Word version

Private Sub Document_Open()
    MsgBox ""#,
    output_for_hello_world!(),
    r#""
End Sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VbaWord,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VbaWord,
            syntax: VBA_WORD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VBA_WORD,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"' ",
                                output_for_hello_world!(),
                                r" in Visual Basic for Applications, Word version"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"Private"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"Sub"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Document_Open"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"MsgBox"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"End"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"Sub"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Lpc`.
//!
//! Imported from the Hello World Collection (`files/LPC.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LPC.htm>
//! - <http://helloworldcollection.de/#LPC>
//! - <https://www.wikidata.org/wiki/Q1798284> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, ObjectOriented, Procedural, PrototypeBased,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lpc => "LPC",
    versions: [Lpc => "LPC"],
    extensions: [],
}

const LPC: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in LPC

void create()
{
       message("info",""#,
    output_for_hello_world!(),
    r#"",this_user());
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lpc,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, ObjectOriented, PrototypeBased, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lpc,
            syntax: LPC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LPC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"// ", output_for_hello_world!(), r" in LPC"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"create"),
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
                        (TokenKind::Function, r"message"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""info""#),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Function, r"this_user"),
                        (TokenKind::Punctuation, r"());"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };

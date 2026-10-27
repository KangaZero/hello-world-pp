//! Hello-world snippet for `ProgrammingLanguageName::Macro10`.
//!
//! Imported from the Hello World Collection (`files/MACRO-10.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MACRO-10.txt>
//! - <http://helloworldcollection.de/#MACRO-10>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Macro10 => "MACRO-10",
    versions: [Macro10 => "MACRO-10"],
    extensions: [],
}

const MACRO10: &str = concat!(
    r"TITLE ",
    output_for_hello_world!(),
    r"
; ",
    output_for_hello_world!(),
    r" IN MACRO 10 FOR TOPS-10
ENTRY OUTPUT
SEARCH UUOSYM

LAB:    ASCIZ /",
    output_for_hello_world!(),
    r"
/
OUTPUT: OUTSTR LAB              ; OUTPUT MESSAGE
        MONRT.                  ; RETURN TO MONITOR          
        END OUTPUT"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Macro10,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Macro10,
            syntax: MACRO10,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MACRO10,
                    [
                        (TokenKind::Keyword, r"TITLE"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" ",
                                output_for_hello_world!(),
                                r"
"
                            ),
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" IN MACRO 10 FOR TOPS-10"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"ENTRY"),
                        (
                            TokenKind::Plain,
                            r" OUTPUT
",
                        ),
                        (TokenKind::Keyword, r"SEARCH"),
                        (
                            TokenKind::Plain,
                            r" UUOSYM

LAB",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"    "),
                        (TokenKind::Keyword, r"ASCIZ"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" /",
                                output_for_hello_world!(),
                                r"
/
OUTPUT"
                            ),
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" OUTSTR LAB              "),
                        (TokenKind::Comment, r"; OUTPUT MESSAGE"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MONRT"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"                  "),
                        (TokenKind::Comment, r"; RETURN TO MONITOR          "),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" OUTPUT"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Clips`.
//!
//! Imported from leachim6/hello-world (`c/CLIPS.clips` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CLIPS.clips>
//! - <https://www.wikidata.org/wiki/Q165372> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Clips => "CLIPS",
    versions: [Clips => "CLIPS"],
    extensions: [Clips => "clips"],
}

const CLIPS: &str = concat!(
    r#"(defrule hw
    (f ?x)
=>
    (printout t ?x crlf))

(assert (f ""#,
    output_for_hello_world!(),
    r#""))

(run)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clips,
        extensions: &[FileExtension::Clips],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clips,
            syntax: CLIPS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLIPS,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"defrule"),
                        (
                            TokenKind::Plain,
                            r" hw
    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"f"),
                        (TokenKind::Plain, r" ?x"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
=>
    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"printout"),
                        (TokenKind::Plain, r" t ?x crlf"),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"assert"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"f"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"run"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

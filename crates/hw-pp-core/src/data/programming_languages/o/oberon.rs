//! Hello-world snippet for `ProgrammingLanguageName::Oberon`.
//!
//! Imported from leachim6/hello-world (`o/Oberon.o` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Oberon.o>
//! - <https://www.wikidata.org/wiki/Q1323362> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Modula, Oberon},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Oberon => "Oberon",
    versions: [Oberon => "Oberon"],
    extensions: [O => "o"],
}

const OBERON: &str = concat!(
    r"MODULE HelloWorld;

IMPORT Out;

BEGIN
   Out.Open;
   Out.String('",
    output_for_hello_world!(),
    r"');
END HelloWorld."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oberon,
        extensions: &[FileExtension::O],
        categories: &[Compiled, Historical],
        families: &[Modula, Oberon],
        paradigms: &[Imperative, Procedural, Structured, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oberon,
            syntax: OBERON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBERON,
                    [
                        (TokenKind::Keyword, r"MODULE"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"IMPORT"),
                        (TokenKind::Plain, r" Out"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"BEGIN"),
                        (
                            TokenKind::Plain,
                            r"
   Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Open"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
   Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r"."),
                    ],
                )
            },
        }],
    };

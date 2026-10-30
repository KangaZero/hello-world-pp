//! Hello-world snippet for `ProgrammingLanguageName::Lisp`.
//!
//! Imported from leachim6/hello-world (`l/Lisp.lsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lisp.lsp>
//! - <https://www.wikidata.org/wiki/Q132874> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Procedural, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Lisp => "Lisp",
    versions: [Lisp => "Lisp"],
    extensions: [],
}

const LISP: &str = r"; LISP
(DEFUN hello ()
  (PRINT (LIST 'HELLO 'WORLD))
)

(hello)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lisp,
        extensions: &[],
        categories: &[Scripting, Historical],
        families: &[Lisp],
        paradigms: &[Procedural, Functional, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lisp,
            syntax: LISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LISP,
                    [
                        (
                            TokenKind::Comment,
                            r"; LISP
",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Type, r"DEFUN"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"PRINT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"LIST"),
                        (TokenKind::Plain, r" 'HELLO 'WORLD"),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"hello"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

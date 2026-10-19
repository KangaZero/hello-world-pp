//! Hello-world snippet for `ProgrammingLanguageName::Lisp`.
//!
//! Imported from leachim6/hello-world (`l/Lisp.lsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lisp.lsp>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Lisp => "Lisp",
    versions: [Lisp => "Lisp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lisp,
        extensions: &[],
        categories: &[Scripting, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lisp,
            syntax: r"; LISP
(DEFUN hello ()
  (PRINT (LIST 'HELLO 'WORLD))
)

(hello)",
            tokenized_syntax: &[
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
        }],
    };

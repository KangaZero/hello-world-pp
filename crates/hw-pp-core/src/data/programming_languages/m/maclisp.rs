//! Hello-world snippet for `ProgrammingLanguageName::Maclisp`.
//!
//! Imported from leachim6/hello-world (`m/MacLisp.lisp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MacLisp.lisp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Maclisp => "MacLisp",
    versions: [Maclisp => "MacLisp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maclisp,
        extensions: &[],
        categories: &[Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maclisp,
            syntax: concat!(
                r#"(comment) ;-*- Lisp -*-
(progn
  (defun hello-world ()
    (princ ""#,
                output_for_hello_world!(),
                r#"")
    (quit))
  (close (prog1 infile (inpush -1)))
  (sstatus feature noldmsg)
  (gctwa)
  (gc)
  (sstatus flush t)
  (suspend ":KILL " '(ts hello))
  (hello-world))"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"comment"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::Comment,
                    r";-*- Lisp -*-
",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"progn"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"defun"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"hello-world"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"princ "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"quit"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"close"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"prog1"),
                (TokenKind::Plain, r" infile "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"inpush "),
                (TokenKind::Keyword, r"-"),
                (TokenKind::Number, r"1"),
                (TokenKind::Punctuation, r")))"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"sstatus feature noldmsg"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"gctwa"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"gc"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"sstatus flush t"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"suspend "),
                (TokenKind::String, r#"":KILL ""#),
                (TokenKind::Plain, r" '"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"ts hello"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"hello"),
                (TokenKind::Keyword, r"-"),
                (TokenKind::Plain, r"world"),
                (TokenKind::Punctuation, r"))"),
            ],
        }],
    };

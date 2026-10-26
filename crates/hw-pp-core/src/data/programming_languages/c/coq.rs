//! Hello-world snippet for `ProgrammingLanguageName::Coq`.
//!
//! Imported from leachim6/hello-world (`c/Coq.v` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Coq.v>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::ProofAssistant,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coq => "Coq",
    versions: [Coq => "Coq"],
    extensions: [V => "v"],
}

const COQ: &str = concat!(
    r"Require Import Coq.Lists.List.
Require Import Io.All.
Require Import Io.System.All.
Require Import ListString.All.

Import ListNotations.
Import C.Notations.

(** The classic ",
    output_for_hello_world!(),
    r#" program. *)
Definition hello_world (argv : list LString.t) : C.t System.effect unit :=
  System.log (LString.s ""#,
    output_for_hello_world!(),
    r#"")."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coq,
        extensions: &[FileExtension::V],
        categories: &[ProofAssistant],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coq,
            syntax: COQ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COQ,
                    [
                        (TokenKind::Plain, r"Require Import Coq"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Lists"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"List"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
Require Import Io",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"All"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
Require Import Io",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"System"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"All"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
Require Import ListString",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"All"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"

Import ListNotations",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
Import C",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Notations"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"(** The classic ",
                                output_for_hello_world!(),
                                r" program. *)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"Definition"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello_world"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"argv "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" list LString"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"t"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" C"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"t System"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"effect "),
                        (TokenKind::Type, r"unit"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"=
  System",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"LString"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"s "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")."),
                    ],
                )
            },
        }],
    };

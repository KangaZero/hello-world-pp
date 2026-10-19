//! Hello-world snippet for `ProgrammingLanguageName::Coq`.
//!
//! Imported from leachim6/hello-world (`c/Coq.v` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Coq => "Coq",
    versions: [Coq => "Coq"],
    extensions: [V => "v"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coq,
        extensions: &[FileExtension::V],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coq,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };

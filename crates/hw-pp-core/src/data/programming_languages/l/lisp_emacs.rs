//! Hello-world snippet for `ProgrammingLanguageName::LispEmacs`.
//!
//! Imported from the Hello World Collection (`files/Lisp-Emacs.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Lisp-Emacs.txt>
//! - <http://helloworldcollection.de/#Lisp-Emacs>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: LispEmacs => "Lisp-Emacs",
    versions: [LispEmacs => "Lisp-Emacs"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LispEmacs,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LispEmacs,
            syntax: concat!(
                r";;; ",
                output_for_hello_world!(),
                r#" in Emacs Lisp.

(defun hello-world()
  "Display the string "#,
                output_for_hello_world!(),
                r#"."
  (interactive)
  (message ""#,
                output_for_hello_world!(),
                r#""))"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";;; ",
                    output_for_hello_world!(),
                    r#" in Emacs Lisp.

(defun hello-world()
  "Display the string "#,
                    output_for_hello_world!(),
                    r#"."
  (interactive)
  (message ""#,
                    output_for_hello_world!(),
                    r#""))"#
                ),
            )],
        }],
    };

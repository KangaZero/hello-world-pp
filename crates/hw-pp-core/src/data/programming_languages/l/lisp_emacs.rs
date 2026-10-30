//! Hello-world snippet for `ProgrammingLanguageName::LispEmacs`.
//!
//! Imported from the Hello World Collection (`files/Lisp-Emacs.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Lisp-Emacs.txt>
//! - <http://helloworldcollection.de/#Lisp-Emacs>
//! - <https://en.wikipedia.org/wiki/Emacs_Lisp> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: LispEmacs => "Lisp-Emacs",
    versions: [LispEmacs => "Lisp-Emacs"],
    extensions: [],
}

const LISP_EMACS: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LispEmacs,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Functional, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LispEmacs,
            syntax: LISP_EMACS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LISP_EMACS,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r";;; ", output_for_hello_world!(), r" in Emacs Lisp."),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"defun"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello-world"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (
                            TokenKind::String,
                            concat!(
                                r#""Display the string "#,
                                output_for_hello_world!(),
                                r#".""#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"interactive"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"message"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"))"),
                    ],
                )
            },
        }],
    };

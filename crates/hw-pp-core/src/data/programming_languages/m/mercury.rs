//! Hello-world snippet for `ProgrammingLanguageName::Mercury`.
//!
//! Imported from leachim6/hello-world (`m/Mercury.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mercury => "Mercury",
    versions: [Mercury => "Mercury"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mercury,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mercury,
            syntax: concat!(
                r#":- module hello.
:- interface.
:- import_module io.
:- pred main(io::di, io::uo) is det.

:- implementation.
main(!IO) :-
	io.write_string(""#,
                output_for_hello_world!(),
                r#"\n", !IO)."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#":- module hello.
:- interface.
:- import_module io.
:- pred main(io::di, io::uo) is det.

:- implementation.
main(!IO) :-
	io.write_string(""#,
                    output_for_hello_world!(),
                    r#"\n", !IO)."#
                ),
            )],
        }],
    };

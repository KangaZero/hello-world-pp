//! Hello-world snippet for `ProgrammingLanguageName::Dos`.
//!
//! Imported from leachim6/hello-world (`d/dos.bat` at `a152253`),
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
    language: Dos => "dos",
    versions: [Dos => "dos"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dos,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dos,
            syntax: concat!(r"@echo ", output_for_hello_world!()),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"@echo"),
                (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
            ],
        }],
    };

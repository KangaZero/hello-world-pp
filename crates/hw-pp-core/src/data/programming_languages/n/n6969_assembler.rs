//! Hello-world snippet for `ProgrammingLanguageName::N6969Assembler`.
//!
//! Imported from leachim6/hello-world (`#/6969 Assembler` at `a152253`),
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
    language: N6969Assembler => "6969 Assembler",
    versions: [N6969Assembler => "6969 Assembler"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N6969Assembler,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N6969Assembler,
            syntax: concat!(r"MOV C*::", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"MOV C*::", output_for_hello_world!()),
            )],
        }],
    };

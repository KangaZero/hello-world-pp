//! Hello-world snippet for `ProgrammingLanguageName::GotoPlusPlus`.
//!
//! Imported from leachim6/hello-world (`g/GOTO++.gpp` at `a152253`),
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
    language: GotoPlusPlus => "GOTO++",
    versions: [GotoPlusPlus => "GOTO++"],
    extensions: [Gpp => "gpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GotoPlusPlus,
        extensions: &[FileExtension::Gpp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GotoPlusPlus,
            syntax: concat!(r"GOTOPRINTDUTEXTE() ; «", output_for_hello_world!(), r"»"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"GOTOPRINTDUTEXTE() ; «", output_for_hello_world!(), r"»"),
            )],
        }],
    };

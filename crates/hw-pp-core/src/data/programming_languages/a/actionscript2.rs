//! Hello-world snippet for `ProgrammingLanguageName::Actionscript2`.
//!
//! Imported from leachim6/hello-world (`a/ActionScript 2.as` at `a152253`),
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
    language: Actionscript2 => "ActionScript 2",
    versions: [Actionscript2 => "ActionScript 2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Actionscript2,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Actionscript2,
            syntax: concat!(r#"trace( ""#, output_for_hello_world!(), r#"" );"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"trace"),
                (TokenKind::Plain, r"( "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Plain, r" );"),
            ],
        }],
    };

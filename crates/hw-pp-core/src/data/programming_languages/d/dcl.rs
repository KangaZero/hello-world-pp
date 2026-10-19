//! Hello-world snippet for `ProgrammingLanguageName::Dcl`.
//!
//! Imported from leachim6/hello-world (`d/DCL` at `a152253`),
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
    language: Dcl => "DCL",
    versions: [Dcl => "DCL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dcl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dcl,
            syntax: concat!(r#"WRITE SYS$OUTPUT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"WRITE SYS$OUTPUT ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Rescript`.
//!
//! Imported from leachim6/hello-world (`r/ReScript.res` at `a152253`),
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
    language: Rescript => "ReScript",
    versions: [Rescript => "ReScript"],
    extensions: [Res => "res"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rescript,
        extensions: &[FileExtension::Res],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rescript,
            syntax: concat!(r#"Js.log(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Js.log(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

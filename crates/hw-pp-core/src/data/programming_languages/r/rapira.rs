//! Hello-world snippet for `ProgrammingLanguageName::Rapira`.
//!
//! Imported from leachim6/hello-world (`r/Rapira.rap` at `a152253`),
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
    language: Rapira => "Rapira",
    versions: [Rapira => "Rapira"],
    extensions: [Rap => "rap"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rapira,
        extensions: &[FileExtension::Rap],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rapira,
            syntax: concat!(r#"output: ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"output: ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };

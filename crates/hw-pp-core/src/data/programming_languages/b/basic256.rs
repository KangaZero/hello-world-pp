//! Hello-world snippet for `ProgrammingLanguageName::Basic256`.
//!
//! Imported from leachim6/hello-world (`b/BASIC 256.kbs` at `a152253`),
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
    language: Basic256 => "BASIC 256",
    versions: [Basic256 => "BASIC 256"],
    extensions: [Kbs => "kbs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic256,
        extensions: &[FileExtension::Kbs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Basic256,
            syntax: concat!(r#"Print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

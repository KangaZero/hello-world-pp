//! Hello-world snippet for `ProgrammingLanguageName::Blackrum`.
//!
//! Imported from leachim6/hello-world (`b/BlackRum.br` at `a152253`),
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
    language: Blackrum => "BlackRum",
    versions: [Blackrum => "BlackRum"],
    extensions: [Br => "br"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blackrum,
        extensions: &[FileExtension::Br],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blackrum,
            syntax: concat!(r#"slur ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"slur ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

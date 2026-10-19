//! Hello-world snippet for `ProgrammingLanguageName::Zim`.
//!
//! Imported from leachim6/hello-world (`z/Zim.zim` at `a152253`),
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
    language: Zim => "Zim",
    versions: [Zim => "Zim"],
    extensions: [Zim => "zim"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zim,
        extensions: &[FileExtension::Zim],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zim,
            syntax: concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Burlesque`.
//!
//! Imported from leachim6/hello-world (`b/Burlesque.burlesque` at `a152253`),
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
    language: Burlesque => "Burlesque",
    versions: [Burlesque => "Burlesque"],
    extensions: [Burlesque => "burlesque"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Burlesque,
        extensions: &[FileExtension::Burlesque],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Burlesque,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#""Q"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#""Q"#),
            )],
        }],
    };

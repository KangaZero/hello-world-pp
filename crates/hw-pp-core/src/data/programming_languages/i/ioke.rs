//! Hello-world snippet for `ProgrammingLanguageName::Ioke`.
//!
//! Imported from leachim6/hello-world (`i/Ioke.ik` at `a152253`),
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
    language: Ioke => "Ioke",
    versions: [Ioke => "Ioke"],
    extensions: [Ik => "ik"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ioke,
        extensions: &[FileExtension::Ik],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ioke,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"" println"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"" println"#),
            )],
        }],
    };

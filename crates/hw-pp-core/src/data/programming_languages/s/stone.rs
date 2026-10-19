//! Hello-world snippet for `ProgrammingLanguageName::Stone`.
//!
//! Imported from leachim6/hello-world (`s/Stone.stone` at `a152253`),
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
    language: Stone => "Stone",
    versions: [Stone => "Stone"],
    extensions: [Stone => "stone"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stone,
        extensions: &[FileExtension::Stone],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stone,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

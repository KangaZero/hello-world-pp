//! Hello-world snippet for `ProgrammingLanguageName::Passerine`.
//!
//! Imported from leachim6/hello-world (`p/Passerine.pn` at `a152253`),
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
    language: Passerine => "Passerine",
    versions: [Passerine => "Passerine"],
    extensions: [Pn => "pn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Passerine,
        extensions: &[FileExtension::Pn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Passerine,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

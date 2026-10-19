//! Hello-world snippet for `ProgrammingLanguageName::Gri`.
//!
//! Imported from leachim6/hello-world (`g/Gri.gri` at `a152253`),
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
    language: Gri => "Gri",
    versions: [Gri => "Gri"],
    extensions: [Gri => "gri"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gri,
        extensions: &[FileExtension::Gri],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gri,
            syntax: concat!(r#"show ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"show ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

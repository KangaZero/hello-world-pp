//! Hello-world snippet for `ProgrammingLanguageName::Xtal`.
//!
//! Imported from leachim6/hello-world (`x/Xtal.xtal` at `a152253`),
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
    language: Xtal => "Xtal",
    versions: [Xtal => "Xtal"],
    extensions: [Xtal => "xtal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xtal,
        extensions: &[FileExtension::Xtal],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xtal,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"".p;"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"".p;"#),
            )],
        }],
    };

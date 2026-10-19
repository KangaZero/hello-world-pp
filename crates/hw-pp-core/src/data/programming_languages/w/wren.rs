//! Hello-world snippet for `ProgrammingLanguageName::Wren`.
//!
//! Imported from leachim6/hello-world (`w/Wren.wren` at `a152253`),
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
    language: Wren => "Wren",
    versions: [Wren => "Wren"],
    extensions: [Wren => "wren"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wren,
        extensions: &[FileExtension::Wren],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wren,
            syntax: concat!(r#"IO.print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"IO.print(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

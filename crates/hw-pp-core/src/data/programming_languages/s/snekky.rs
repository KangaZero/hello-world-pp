//! Hello-world snippet for `ProgrammingLanguageName::Snekky`.
//!
//! Imported from leachim6/hello-world (`s/Snekky.snek` at `a152253`),
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
    language: Snekky => "Snekky",
    versions: [Snekky => "Snekky"],
    extensions: [Snek => "snek"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snekky,
        extensions: &[FileExtension::Snek],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snekky,
            syntax: concat!(r#"Sys.println(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Sys.println(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

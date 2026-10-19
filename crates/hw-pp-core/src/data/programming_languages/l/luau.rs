//! Hello-world snippet for `ProgrammingLanguageName::Luau`.
//!
//! Imported from leachim6/hello-world (`l/Luau.luau` at `a152253`),
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
    language: Luau => "Luau",
    versions: [Luau => "Luau"],
    extensions: [Luau => "luau"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Luau,
        extensions: &[FileExtension::Luau],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Luau,
            syntax: concat!(r"print('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print('", output_for_hello_world!(), r"')"),
            )],
        }],
    };

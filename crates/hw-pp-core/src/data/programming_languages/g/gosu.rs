//! Hello-world snippet for `ProgrammingLanguageName::Gosu`.
//!
//! Imported from leachim6/hello-world (`g/Gosu.gosu` at `a152253`, `g/Gosu.gs` at `a152253`),
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
    language: Gosu => "Gosu",
    versions: [GosuGosu => "Gosu (.gosu)", GosuGs => "Gosu (.gs)"],
    extensions: [Gosu => "gosu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gosu,
        extensions: &[FileExtension::Gosu],
        categories: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::GosuGosu,
                syntax: concat!(r#"print(""#, output_for_hello_world!(), r#" from Gosu!!")"#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"print(""#, output_for_hello_world!(), r#" from Gosu!!")"#),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::GosuGs,
                syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                )],
            },
        ],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Leekscript`.
//!
//! Imported from leachim6/hello-world (PR #1533 `l/LeekScript.leek`),
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
    language: Leekscript => "LeekScript",
    versions: [Leekscript => "LeekScript"],
    extensions: [Leek => "leek"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Leekscript,
        extensions: &[FileExtension::Leek],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Leekscript,
            syntax: concat!(r#"say(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"say(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

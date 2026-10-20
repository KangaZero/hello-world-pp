//! Hello-world snippet for `ProgrammingLanguageName::ControlLanguage`.
//!
//! Imported from leachim6/hello-world (`c/Control Language.cllc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Control%20Language.cllc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ControlLanguage => "Control Language",
    versions: [ControlLanguage => "Control Language"],
    extensions: [Cllc => "cllc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ControlLanguage,
        extensions: &[FileExtension::Cllc],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ControlLanguage,
            syntax: concat!(r#"SNDPGMMSG MSG(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"SNDPGMMSG MSG(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

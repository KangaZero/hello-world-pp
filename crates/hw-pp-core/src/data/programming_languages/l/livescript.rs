//! Hello-world snippet for `ProgrammingLanguageName::Livescript`.
//!
//! Imported from leachim6/hello-world (`l/LiveScript.ls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LiveScript.ls>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Livescript => "LiveScript",
    versions: [Livescript => "LiveScript"],
    extensions: [Ls => "ls"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Livescript,
        extensions: &[FileExtension::Ls],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Livescript,
            syntax: concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

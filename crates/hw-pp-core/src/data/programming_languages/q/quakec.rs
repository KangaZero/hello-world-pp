//! Hello-world snippet for `ProgrammingLanguageName::Quakec`.
//!
//! Imported from leachim6/hello-world (`q/QuakeC.qc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QuakeC.qc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Quakec => "QuakeC",
    versions: [Quakec => "QuakeC"],
    extensions: [Qc => "qc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quakec,
        extensions: &[FileExtension::Qc],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quakec,
            syntax: concat!(r#"bprint(""#, output_for_hello_world!(), r#"\n");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"bprint(""#, output_for_hello_world!(), r#"\n");"#),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Quakec`.
//!
//! Imported from leachim6/hello-world (`q/QuakeC.qc` at `a152253`),
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
    language: Quakec => "QuakeC",
    versions: [Quakec => "QuakeC"],
    extensions: [Qc => "qc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quakec,
        extensions: &[FileExtension::Qc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quakec,
            syntax: concat!(r#"bprint(""#, output_for_hello_world!(), r#"\n");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"bprint(""#, output_for_hello_world!(), r#"\n");"#),
            )],
        }],
    };

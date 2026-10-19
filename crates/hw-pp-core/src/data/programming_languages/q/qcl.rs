//! Hello-world snippet for `ProgrammingLanguageName::Qcl`.
//!
//! Imported from leachim6/hello-world (`q/QCL.qcl` at `a152253`),
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
    language: Qcl => "QCL",
    versions: [Qcl => "QCL"],
    extensions: [Qcl => "qcl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qcl,
        extensions: &[FileExtension::Qcl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qcl,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };

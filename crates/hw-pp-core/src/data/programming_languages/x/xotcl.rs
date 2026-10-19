//! Hello-world snippet for `ProgrammingLanguageName::Xotcl`.
//!
//! Imported from leachim6/hello-world (`x/XOTcl.xotcl` at `a152253`),
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
    language: Xotcl => "XOTcl",
    versions: [Xotcl => "XOTcl"],
    extensions: [Xotcl => "xotcl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xotcl,
        extensions: &[FileExtension::Xotcl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xotcl,
            syntax: concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

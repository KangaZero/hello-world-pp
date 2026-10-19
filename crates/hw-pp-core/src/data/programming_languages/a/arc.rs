//! Hello-world snippet for `ProgrammingLanguageName::Arc`.
//!
//! Imported from leachim6/hello-world (`a/ARC.arc` at `a152253`),
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
    language: Arc => "ARC",
    versions: [Arc => "ARC"],
    extensions: [Arc => "arc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arc,
        extensions: &[FileExtension::Arc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arc,
            syntax: concat!(r#"(prn ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(prn ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

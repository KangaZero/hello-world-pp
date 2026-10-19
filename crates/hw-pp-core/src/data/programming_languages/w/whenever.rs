//! Hello-world snippet for `ProgrammingLanguageName::Whenever`.
//!
//! Imported from leachim6/hello-world (`w/Whenever.whenever` at `a152253`),
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
    language: Whenever => "Whenever",
    versions: [Whenever => "Whenever"],
    extensions: [Whenever => "whenever"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whenever,
        extensions: &[FileExtension::Whenever],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whenever,
            syntax: concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#),
            )],
        }],
    };

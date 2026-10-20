//! Hello-world snippet for `ProgrammingLanguageName::Whenever`.
//!
//! Imported from leachim6/hello-world (`w/Whenever.whenever` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Whenever.whenever>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whenever,
            syntax: concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#),
            )],
        }],
    };

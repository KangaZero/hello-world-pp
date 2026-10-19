//! Hello-world snippet for `ProgrammingLanguageName::Cangjie`.
//!
//! Imported from leachim6/hello-world (PR #1823 `c/Cangjie.cj`),
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
    language: Cangjie => "Cangjie",
    versions: [Cangjie => "Cangjie"],
    extensions: [Cj => "cj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cangjie,
        extensions: &[FileExtension::Cj],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cangjie,
            syntax: concat!(
                r#"main(): Int64 {
    println(""#,
                output_for_hello_world!(),
                r#"")
    return 0
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main(): Int64 {
    println(""#,
                    output_for_hello_world!(),
                    r#"")
    return 0
}"#
                ),
            )],
        }],
    };

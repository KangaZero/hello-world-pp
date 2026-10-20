//! Hello-world snippet for `ProgrammingLanguageName::AddPlusPlus`.
//!
//! Imported from leachim6/hello-world (`a/Add++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Add%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AddPlusPlus => "Add++",
    versions: [AddPlusPlus => "Add++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AddPlusPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AddPlusPlus,
            syntax: concat!(
                r#"D,f,^,""#,
                output_for_hello_world!(),
                r#""
$f
O"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"D,f,^,""#,
                    output_for_hello_world!(),
                    r#""
$f
O"#
                ),
            )],
        }],
    };

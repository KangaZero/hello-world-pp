//! Hello-world snippet for `ProgrammingLanguageName::Milkscript`.
//!
//! Imported from leachim6/hello-world (`m/MilkScript.milk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MilkScript.milk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Milkscript => "MilkScript",
    versions: [Milkscript => "MilkScript"],
    extensions: [Milk => "milk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Milkscript,
        extensions: &[FileExtension::Milk],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Milkscript,
            syntax: concat!(
                r#"#import "basic"
print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#import "basic"
print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };

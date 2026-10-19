//! Hello-world snippet for `ProgrammingLanguageName::Miranda`.
//!
//! Imported from leachim6/hello-world (`m/Miranda.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Miranda.m>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Miranda => "Miranda",
    versions: [Miranda => "Miranda"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Miranda,
        extensions: &[],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Miranda,
            syntax: concat!(
                r#"main :: [sys_message]
main = [Stdout ""#,
                output_for_hello_world!(),
                r#""]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main :: [sys_message]
main = [Stdout ""#,
                    output_for_hello_world!(),
                    r#""]"#
                ),
            )],
        }],
    };

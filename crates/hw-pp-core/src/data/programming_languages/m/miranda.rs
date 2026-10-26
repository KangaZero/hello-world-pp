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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Miranda => "Miranda",
    versions: [Miranda => "Miranda"],
    extensions: [],
}

const MIRANDA: &str = concat!(
    r#"main :: [sys_message]
main = [Stdout ""#,
    output_for_hello_world!(),
    r#""]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Miranda,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Miranda,
            syntax: MIRANDA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIRANDA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"main :: [sys_message]
main = [Stdout ""#,
                            output_for_hello_world!(),
                            r#""]"#
                        ),
                    )],
                )
            },
        }],
    };

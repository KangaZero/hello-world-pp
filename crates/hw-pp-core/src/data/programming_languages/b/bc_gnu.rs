//! Hello-world snippet for `ProgrammingLanguageName::BcGnu`.
//!
//! Imported from leachim6/hello-world (`b/BC GNU.bc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BC%20GNU.bc>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BcGnu => "BC GNU",
    versions: [BcGnu => "BC GNU"],
    extensions: [],
}

const BC_GNU: &str = concat!(
    r#"print ""#,
    output_for_hello_world!(),
    r#""
quit"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BcGnu,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BcGnu,
            syntax: BC_GNU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BC_GNU,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"print ""#,
                            output_for_hello_world!(),
                            r#""
quit"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Ufe65Ufe64Ufe65`.
//!
//! Imported from leachim6/hello-world (`#/﹥﹤﹥` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%EF%B9%A5%EF%B9%A4%EF%B9%A5>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ufe65Ufe64Ufe65 => "﹥﹤﹥",
    versions: [Ufe65Ufe64Ufe65 => "﹥﹤﹥"],
    extensions: [],
}

const UFE65_UFE64_UFE65: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#""r\
          o;!?l<"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ufe65Ufe64Ufe65,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ufe65Ufe64Ufe65,
            syntax: UFE65_UFE64_UFE65,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UFE65_UFE64_UFE65,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#""r\
          o;!?l<"#
                        ),
                    )],
                )
            },
        }],
    };

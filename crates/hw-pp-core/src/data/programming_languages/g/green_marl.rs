//! Hello-world snippet for `ProgrammingLanguageName::GreenMarl`.
//!
//! Imported from leachim6/hello-world (`g/Green Marl.gm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Green%20Marl.gm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: GreenMarl => "Green Marl",
    versions: [GreenMarl => "Green Marl"],
    extensions: [],
}

const GREEN_MARL: &str = concat!(
    r#"procedure hello_world() {
  println(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GreenMarl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GreenMarl,
            syntax: GREEN_MARL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GREEN_MARL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"procedure hello_world() {
  println(""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };

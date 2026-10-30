//! Hello-world snippet for `ProgrammingLanguageName::Skip`.
//!
//! Imported from leachim6/hello-world (`s/Skip.sk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Skip.sk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Skip => "Skip",
    versions: [Skip => "Skip"],
    extensions: [Sk => "sk"],
}

const SKIP: &str = concat!(
    r#"fun main(): void {
  print_string(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Skip,
        extensions: &[FileExtension::Sk],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Skip,
            syntax: SKIP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SKIP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"fun main(): void {
  print_string(""#,
                            output_for_hello_world!(),
                            r#"")
}"#
                        ),
                    )],
                )
            },
        }],
    };

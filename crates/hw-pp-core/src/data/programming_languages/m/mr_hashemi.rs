//! Hello-world snippet for `ProgrammingLanguageName::MrHashemi`.
//!
//! Imported from leachim6/hello-world (`m/Mr Hashemi.hashemi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mr%20Hashemi.hashemi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: MrHashemi => "Mr Hashemi",
    versions: [MrHashemi => "Mr Hashemi"],
    extensions: [Hashemi => "hashemi"],
}

const MR_HASHEMI: &str = concat!(
    r#"bebin azinja() {
  bechap(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MrHashemi,
        extensions: &[FileExtension::Hashemi],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MrHashemi,
            syntax: MR_HASHEMI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MR_HASHEMI,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"bebin azinja() {
  bechap(""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };

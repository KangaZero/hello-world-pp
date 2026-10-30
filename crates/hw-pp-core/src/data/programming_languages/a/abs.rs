//! Hello-world snippet for `ProgrammingLanguageName::Abs`.
//!
//! Imported from leachim6/hello-world (`a/ABS.abs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ABS.abs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Abs => "ABS",
    versions: [Abs => "ABS"],
    extensions: [Abs => "abs"],
}

const ABS: &str = concat!(r#"echo(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Abs,
        extensions: &[FileExtension::Abs],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Abs,
            syntax: ABS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ABS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"echo(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

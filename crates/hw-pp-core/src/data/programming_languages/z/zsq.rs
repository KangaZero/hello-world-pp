//! Hello-world snippet for `ProgrammingLanguageName::Zsq`.
//!
//! Imported from leachim6/hello-world (`z/Zsq.zsq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zsq.zsq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zsq => "Zsq",
    versions: [Zsq => "Zsq"],
    extensions: [Zsq => "zsq"],
}

const ZSQ: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zsq,
        extensions: &[FileExtension::Zsq],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zsq,
            syntax: ZSQ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZSQ,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

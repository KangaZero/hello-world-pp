//! Hello-world snippet for `ProgrammingLanguageName::Pythowo`.
//!
//! Imported from leachim6/hello-world (`p/pythOwO.pyowo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/pythOwO.pyowo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pythowo => "pythOwO",
    versions: [Pythowo => "pythOwO"],
    extensions: [Pyowo => "pyowo"],
}

const PYTHOWO: &str = concat!(r#"pwint(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pythowo,
        extensions: &[FileExtension::Pyowo],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pythowo,
            syntax: PYTHOWO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PYTHOWO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"pwint(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

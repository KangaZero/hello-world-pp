//! Hello-world snippet for `ProgrammingLanguageName::Zlang`.
//!
//! Imported from leachim6/hello-world (PR #1552 `z/zlang.zlang`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1552>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zlang => "zlang",
    versions: [Zlang => "zlang"],
    extensions: [Zlang => "zlang"],
}

const ZLANG: &str = concat!(r#"log(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zlang,
        extensions: &[FileExtension::Zlang],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zlang,
            syntax: ZLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

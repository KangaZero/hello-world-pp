//! Hello-world snippet for `ProgrammingLanguageName::Bracketlang`.
//!
//! Imported from leachim6/hello-world (PR #1751 `b/BracketLang.bl`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1751>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bracketlang => "BracketLang",
    versions: [Bracketlang => "BracketLang"],
    extensions: [Bl => "bl"],
}

const BRACKETLANG: &str = concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bracketlang,
        extensions: &[FileExtension::Bl],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bracketlang,
            syntax: BRACKETLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRACKETLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bracketlang => "BracketLang",
    versions: [Bracketlang => "BracketLang"],
    extensions: [Bl => "bl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bracketlang,
        extensions: &[FileExtension::Bl],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bracketlang,
            syntax: concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

//! Hello-world snippets for `ProgrammingLanguageName::Apl`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/APL_(programming_language)>
//! - <https://tryapl.org/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Apl => "APL",
    versions: [Apl => "APL"],
    extensions: [Apl => "apl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &[FileExtension::Apl],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apl,
            syntax: concat!(r"'", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::String,
                concat!("'", output_for_hello_world!(), "'"),
            )],
        }],
    };

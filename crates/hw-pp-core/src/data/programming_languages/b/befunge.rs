//! Hello-world snippets for `ProgrammingLanguageName::Befunge`.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Befunge>
//! - <https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Befunge => "Befunge",
    versions: [Befunge93 => "Befunge-93"],
    extensions: [Bf => "bf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Befunge,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Befunge93,
            syntax: r#""!dlroW ,olleH">:#,_@"#,
            tokenized_syntax: &[
                (TokenKind::String, "\"!dlroW ,olleH\""),
                (TokenKind::Keyword, ">"),
                (TokenKind::Function, ":"),
                (TokenKind::Keyword, "#"),
                (TokenKind::Function, ","),
                (TokenKind::Keyword, "_@"),
            ],
        }],
    };

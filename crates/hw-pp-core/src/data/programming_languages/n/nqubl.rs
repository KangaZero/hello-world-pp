//! Hello-world snippet for `ProgrammingLanguageName::Nqubl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Nqubl>
//! - <https://esolangs.org/wiki/Nqubl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Nqubl => "Nqubl",
    versions: [Nqubl => "Nqubl"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nqubl,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nqubl,
            syntax: r#""!dlrow ,olleH"ex"#,
            tokenized_syntax: &[(TokenKind::Plain, r#""!dlrow ,olleH"ex"#)],
        }],
    };

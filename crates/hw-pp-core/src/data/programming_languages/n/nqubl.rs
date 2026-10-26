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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nqubl => "Nqubl",
    versions: [Nqubl => "Nqubl"],
    extensions: [],
}

const NQUBL: &str = r#""!dlrow ,olleH"ex"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nqubl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nqubl,
            syntax: NQUBL,
            tokenized_syntax: &const {
                checked_token_lengths(NQUBL, [(TokenKind::Plain, r#""!dlrow ,olleH"ex"#)])
            },
        }],
    };

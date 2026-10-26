//! Hello-world snippet for `ProgrammingLanguageName::Smithb`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SMITHb>
//! - <https://esolangs.org/wiki/SMITHb>

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Smithb => "SMITHb",
    versions: [Smithb => "SMITHb"],
    extensions: [],
}

const SMITHB: &str = r#"12(0 *) * *
"!" "d" "l" "r" "o" "W" 32 "o" "l" "l" "e" "H""#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smithb,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smithb,
            syntax: SMITHB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMITHB,
                    [(
                        TokenKind::Plain,
                        r#"12(0 *) * *
"!" "d" "l" "r" "o" "W" 32 "o" "l" "l" "e" "H""#,
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Dfs`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DFS>
//! - <https://esolangs.org/wiki/DFS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dfs => "DFS",
    versions: [Dfs => "DFS"],
    extensions: [],
}

const DFS: &str = r"iiisdsiiiiiiiiOiiiiiiiiiiiiiiiiiiiiiiiiiiiiiOiiiiiiiOOiiiO
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddOddddddddddddO
dddddddddddddddddddddsddOddddddddOiiiOddddddOddddddddO
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddO";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dfs,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dfs,
            syntax: DFS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DFS,
                    [(
                        TokenKind::Plain,
                        r"iiisdsiiiiiiiiOiiiiiiiiiiiiiiiiiiiiiiiiiiiiiOiiiiiiiOOiiiO
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddOddddddddddddO
dddddddddddddddddddddsddOddddddddOiiiOddddddOddddddddO
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddO",
                    )],
                )
            },
        }],
    };

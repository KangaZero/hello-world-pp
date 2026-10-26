//! Hello-world snippet for `ProgrammingLanguageName::BrainFlak`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brain-Flak>
//! - <https://esolangs.org/wiki/Brain-Flak>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BrainFlak => "Brain-Flak",
    versions: [BrainFlak => "Brain-Flak"],
    extensions: [],
}

const BRAIN_FLAK: &str = r"(((((((((((()()()()){}){}){}()))){}{}())[][][][])[][])[[]]())[[][][][][]]())([([]([])[][]{})]()()()([[]](([()()()]([([][][])](((({}()){}))){}{})))))";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BrainFlak,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BrainFlak,
            syntax: BRAIN_FLAK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAIN_FLAK,
                    [(
                        TokenKind::Plain,
                        r"(((((((((((()()()()){}){}){}()))){}{}())[][][][])[][])[[]]())[[][][][][]]())([([]([])[][]{})]()()()([[]](([()()()]([([][][])](((({}()){}))){}{})))))",
                    )],
                )
            },
        }],
    };

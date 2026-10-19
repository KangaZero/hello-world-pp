//! Hello-world snippet for `ProgrammingLanguageName::SmallSCRIPT`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#small_s.c.r.i.p.t.>
//! - <https://esolangs.org/wiki/Small_s.c.r.i.p.t.>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: SmallSCRIPT => "small s.c.r.i.p.t.",
    versions: [SmallSCRIPT => "small s.c.r.i.p.t."],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmallSCRIPT,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmallSCRIPT,
            syntax: r" Hello\, world",
            tokenized_syntax: &[(TokenKind::Plain, r" Hello\, world")],
        }],
    };

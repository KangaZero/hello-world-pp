//! Hello-world snippet for `ProgrammingLanguageName::Pixiedust`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pixiedust>
//! - <https://esolangs.org/wiki/Pixiedust>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Pixiedust => "Pixiedust",
    versions: [Pixiedust => "Pixiedust"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pixiedust,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pixiedust,
            syntax: r"++.*+..+...
++.*++..+.+
++.*++.++..
++.*++.++..
++.*++.++++
++.*+.++..
++.*+.....
++.*+.+.+++
++.*++.++++
++.*+++..+.
++.*++.++..
++.*++..+..
++.*+....+",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"++.*+..+...
++.*++..+.+
++.*++.++..
++.*++.++..
++.*++.++++
++.*+.++..
++.*+.....
++.*+.+.+++
++.*++.++++
++.*+++..+.
++.*++.++..
++.*++..+..
++.*+....+",
            )],
        }],
    };

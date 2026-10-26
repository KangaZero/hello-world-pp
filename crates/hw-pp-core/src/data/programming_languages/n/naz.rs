//! Hello-world snippet for `ProgrammingLanguageName::Naz`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#naz>
//! - <https://esolangs.org/wiki/Naz>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Naz => "naz",
    versions: [Naz => "naz"],
    extensions: [],
}

const NAZ: &str = r"9a8m1o
9a9a9a2a1o
7a2o
3a1o
3d7a1o
9s3s1o
8a2m7a1o
9a9a6a1o
3a1o
6s1o
8s1o
3d1o";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Naz,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Naz,
            syntax: NAZ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NAZ,
                    [(
                        TokenKind::Plain,
                        r"9a8m1o
9a9a9a2a1o
7a2o
3a1o
3d7a1o
9s3s1o
8a2m7a1o
9a9a6a1o
3a1o
6s1o
8s1o
3d1o",
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Fj`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FJ>
//! - <https://esolangs.org/wiki/FJ>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fj => "FJ",
    versions: [Fj => "FJ"],
    extensions: [],
}

const FJ: &str = r"ffffffffffffffffffffffffjjfjfjffffffffffffjjfjfjffffffjfjfffffffffjfjfffffffffjfjfffjffjffjffjffffjjjjjfjfffjfjfffjfjffjjfjjfjfffjjfjffjjjjffffjjjjjfjjfjfjjjfjffjffjffjfjjffffffffffffffffffffffjjfjjffffffffffjjjfjjfjfjjjffffjfjjjfffjjffffffffffjjffjffjffjffjffjffjfjjffjffjffjffjffjffjffjffjfjjjfjjfjffffjjjfjfffffffjj";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fj,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fj,
            syntax: FJ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FJ,
                    [(
                        TokenKind::Plain,
                        r"ffffffffffffffffffffffffjjfjfjffffffffffffjjfjfjffffffjfjfffffffffjfjfffffffffjfjfffjffjffjffjffffjjjjjfjfffjfjfffjfjffjjfjjfjfffjjfjffjjjjffffjjjjjfjjfjfjjjfjffjffjffjfjjffffffffffffffffffffffjjfjjffffffffffjjjfjjfjfjjjffffjfjjjfffjjffffffffffjjffjffjffjffjffjffjfjjffjffjffjffjffjffjffjffjfjjjfjjfjffffjjjfjfffffffjj",
                    )],
                )
            },
        }],
    };

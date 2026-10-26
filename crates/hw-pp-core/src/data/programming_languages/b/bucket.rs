//! Hello-world snippet for `ProgrammingLanguageName::Bucket`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bucket>
//! - <https://esolangs.org/wiki/Bucket>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bucket => "Bucket",
    versions: [Bucket => "Bucket"],
    extensions: [],
}

const BUCKET: &str = r"fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfp
fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpOOfpfp
fpOPePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeP
ePePePePePePePePePePePePeOPePePePePePePePePePePePeOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pOPePePePePePePePeOfpfpfpOPePePePePePeOPePePePePePePePeOPePePePePePePePePePePePePePePePePePePePePePePePePePePePe
PePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeO";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bucket,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bucket,
            syntax: BUCKET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUCKET,
                    [(
                        TokenKind::Plain,
                        r"fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfp
fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpOOfpfp
fpOPePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeP
ePePePePePePePePePePePePeOPePePePePePePePePePePePeOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pOPePePePePePePePeOfpfpfpOPePePePePePeOPePePePePePePePeOPePePePePePePePePePePePePePePePePePePePePePePePePePePePe
PePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeO",
                    )],
                )
            },
        }],
    };

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
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Bucket => "Bucket",
    versions: [Bucket => "Bucket"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bucket,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bucket,
            syntax: r"fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfp
fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpOOfpfp
fpOPePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeP
ePePePePePePePePePePePePeOPePePePePePePePePePePePeOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pOPePePePePePePePeOfpfpfpOPePePePePePeOPePePePePePePePeOPePePePePePePePePePePePePePePePePePePePePePePePePePePePe
PePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeO",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfp
fpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpOfpfpfpfpfpfpfpOOfpfp
fpOPePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeP
ePePePePePePePePePePePePeOPePePePePePePePePePePePeOfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpfpf
pOPePePePePePePePeOfpfpfpOPePePePePePeOPePePePePePePePeOPePePePePePePePePePePePePePePePePePePePePePePePePePePePe
PePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePePeO",
            )],
        }],
    };

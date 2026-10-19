//! Hello-world snippet for `ProgrammingLanguageName::CvNC`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CV(N)(C)>
//! - <https://esolangs.org/wiki/CV(N)(C)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: CvNC => "CV(N)(C)",
    versions: [CvNC => "CV(N)(C)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CvNC,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CvNC,
            syntax: r"cicidigæducədəcədəcədəcədəcəfodigiducificiʔiciʔicidifufiʡiʡifocədəgəduqəʔəcədəgəfoduʡəʡəʡəcəfodogiducidigædəbəfodubifiditifoducədəfogæfodufoboʔidiʡuʔiʔicif",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"cicidigæducədəcədəcədəcədəcəfodigiducificiʔiciʔicidifufiʡiʡifocədəgəduqəʔəcədəgəfoduʡəʡəʡəcəfodogiducidigædəbəfodubifiditifoducədəfogæfodufoboʔidiʡuʔiʔicif",
            )],
        }],
    };

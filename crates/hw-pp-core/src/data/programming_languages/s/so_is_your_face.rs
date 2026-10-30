//! Hello-world snippet for `ProgrammingLanguageName::SoIsYourFace`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#So_is_your_face.>
//! - <https://esolangs.org/wiki/So_is_your_face.>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: SoIsYourFace => "So is your face.",
    versions: [SoIsYourFace => "So is your face."],
    extensions: [],
}

const SO_IS_YOUR_FACE: &str = concat!(
    output_for_hello_world!(),
    r"
So is your face."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SoIsYourFace,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SoIsYourFace,
            syntax: SO_IS_YOUR_FACE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SO_IS_YOUR_FACE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r"
So is your face."
                        ),
                    )],
                )
            },
        }],
    };

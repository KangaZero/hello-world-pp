//! Hello-world snippet for `ProgrammingLanguageName::Dogo`.
//!
//! Imported from leachim6/hello-world (`d/DOGO.dogo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DOGO.dogo>
//! - <https://esolangs.org/wiki/DOGO> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dogo => "DOGO",
    versions: [Dogo => "DOGO"],
    extensions: [Dogo => "dogo"],
}

const DOGO: &str = r"roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over 
roll-over heel roll-over roll-over roll-over heel roll-over roll-over roll-over 
roll-over heel heel heel heel heel heel heel sit roll-over roll-over roll-over 
heel roll-over roll-over roll-over heel heel heel heel roll-over roll-over heel 
roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over  
roll-over roll-over heel roll-over roll-over roll-over roll-over roll-over heel 
roll-over heel heel heel heel heel heel heel roll-over roll-over roll-over 
roll-over roll-over heel heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel heel 
roll-over roll-over roll-over roll-over heel heel heel heel heel heel heel heel 
sit roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel 
heel heel roll-over roll-over heel roll-over roll-over roll-over roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over stay roll-over 
roll-over roll-over heel roll-over roll-over heel roll-over roll-over roll-over 
heel heel heel roll-over roll-over roll-over roll-over heel heel heel heel heel 
heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel heel roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over heel roll-over 
roll-over roll-over roll-over roll-over stay roll-over roll-over roll-over heel 
roll-over roll-over roll-over roll-over heel heel heel roll-over roll-over 
roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel roll-over roll-over heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over heel heel heel heel heel 
heel roll-over roll-over roll-over roll-over heel roll-over roll-over heel heel 
heel heel heel heel heel heel roll-over roll-over roll-over roll-over heel 
roll-over roll-over roll-over heel heel roll-over roll-over roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dogo,
        extensions: &[FileExtension::Dogo],
        categories: &[],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dogo,
            syntax: DOGO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DOGO,
                    [(
                        TokenKind::Plain,
                        r"roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over 
roll-over heel roll-over roll-over roll-over heel roll-over roll-over roll-over 
roll-over heel heel heel heel heel heel heel sit roll-over roll-over roll-over 
heel roll-over roll-over roll-over heel heel heel heel roll-over roll-over heel 
roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over  
roll-over roll-over heel roll-over roll-over roll-over roll-over roll-over heel 
roll-over heel heel heel heel heel heel heel roll-over roll-over roll-over 
roll-over roll-over heel heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel heel 
roll-over roll-over roll-over roll-over heel heel heel heel heel heel heel heel 
sit roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel 
heel heel roll-over roll-over heel roll-over roll-over roll-over roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over stay roll-over 
roll-over roll-over heel roll-over roll-over heel roll-over roll-over roll-over 
heel heel heel roll-over roll-over roll-over roll-over heel heel heel heel heel 
heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel heel roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over heel roll-over 
roll-over roll-over roll-over roll-over stay roll-over roll-over roll-over heel 
roll-over roll-over roll-over roll-over heel heel heel roll-over roll-over 
roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel roll-over roll-over heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over heel heel heel heel heel 
heel roll-over roll-over roll-over roll-over heel roll-over roll-over heel heel 
heel heel heel heel heel heel roll-over roll-over roll-over roll-over heel 
roll-over roll-over roll-over heel heel roll-over roll-over roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel",
                    )],
                )
            },
        }],
    };

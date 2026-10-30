//! Hello-world snippet for `ProgrammingLanguageName::Poetic`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Poetic>
//! - <https://esolangs.org/wiki/Poetic>
//! - <https://esolangs.org/wiki/Poetic_(esolang)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Poetic => "Poetic",
    versions: [Poetic => "Poetic"],
    extensions: [],
}

const POETIC: &str = r#"the proverbial "unconsciousness"

i was already aware
i had understood fully

i saw the devil
i was perfectly still
involuntarily i paused

there said i:
my sheer consciousness
of certain given circumstances
i noticed
it's nothing
     nothing
     nothing any man wouldn't learn

a way of finding these
i know not

nothing common or typical
and yet (somehow)
very little thought
will normally resolve every contradiction

a foolish heart -> an eternal misfortune"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Poetic,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Poetic,
            syntax: POETIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POETIC,
                    [(
                        TokenKind::Plain,
                        r#"the proverbial "unconsciousness"

i was already aware
i had understood fully

i saw the devil
i was perfectly still
involuntarily i paused

there said i:
my sheer consciousness
of certain given circumstances
i noticed
it's nothing
     nothing
     nothing any man wouldn't learn

a way of finding these
i know not

nothing common or typical
and yet (somehow)
very little thought
will normally resolve every contradiction

a foolish heart -> an eternal misfortune"#,
                    )],
                )
            },
        }],
    };

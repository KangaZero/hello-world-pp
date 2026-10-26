//! Hello-world snippet for `ProgrammingLanguageName::Messenger`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Messenger>
//! - <https://esolangs.org/wiki/Messenger>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Messenger => "Messenger",
    versions: [Messenger => "Messenger"],
    extensions: [],
}

const MESSENGER: &str = r"v>8v> *v>v >v >v >v>v >v  >v >v >v >v >v >v >v >v >v >v
>S    7>S>/S1>S>+S>S3>S>/ S7>S>-S>*S>-S>*S>/S>+S>-S>-S>v
 >9*^S^ 5  >+ 7    >+ 3 v S+ 5   3  9  9  7  3  6  8  3/
  vS ^  >^ vS^>^vS>vS^>^>   ^>^vS^ >^vS^ >^vS^vS^vS^vS^";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Messenger,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Messenger,
            syntax: MESSENGER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MESSENGER,
                    [(
                        TokenKind::Plain,
                        r"v>8v> *v>v >v >v >v>v >v  >v >v >v >v >v >v >v >v >v >v
>S    7>S>/S1>S>+S>S3>S>/ S7>S>-S>*S>-S>*S>/S>+S>-S>-S>v
 >9*^S^ 5  >+ 7    >+ 3 v S+ 5   3  9  9  7  3  6  8  3/
  vS ^  >^ vS^>^vS>vS^>^>   ^>^vS^ >^vS^ >^vS^vS^vS^vS^",
                    )],
                )
            },
        }],
    };

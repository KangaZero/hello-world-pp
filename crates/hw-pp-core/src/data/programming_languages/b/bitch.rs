//! Hello-world snippet for `ProgrammingLanguageName::Bitch`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#bitch>
//! - <https://esolangs.org/wiki/Bitch>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Bitch => "bitch",
    versions: [Bitch => "bitch"],
    extensions: [],
}

const BITCH: &str = r"#72/#101/#108/#108/#111/#44/#32/#119/#111/#114/#108/#100/#33/";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bitch,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bitch,
            syntax: BITCH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BITCH,
                    [(
                        TokenKind::Plain,
                        r"#72/#101/#108/#108/#111/#44/#32/#119/#111/#114/#108/#100/#33/",
                    )],
                )
            },
        }],
    };

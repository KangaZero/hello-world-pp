//! Hello-world snippet for `ProgrammingLanguageName::Xeec`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#xEec>
//! - <https://esolangs.org/wiki/XEec>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Xeec => "xEec",
    versions: [Xeec => "xEec"],
    extensions: [],
}

const XEEC: &str = r"h#10 h$! h$d h$l h$r h$o h$W h#32 h$, h$o h$l h$l h$e h$H >o o$ p jno";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xeec,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xeec,
            syntax: XEEC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XEEC,
                    [(
                        TokenKind::Plain,
                        r"h#10 h$! h$d h$l h$r h$o h$W h#32 h$, h$o h$l h$l h$e h$H >o o$ p jno",
                    )],
                )
            },
        }],
    };

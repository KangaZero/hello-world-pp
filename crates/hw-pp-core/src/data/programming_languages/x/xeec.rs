//! Hello-world snippet for `ProgrammingLanguageName::Xeec`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#xEec>
//! - <https://esolangs.org/wiki/XEec>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Xeec => "xEec",
    versions: [Xeec => "xEec"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xeec,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xeec,
            syntax: r"h#10 h$! h$d h$l h$r h$o h$W h#32 h$, h$o h$l h$l h$e h$H >o o$ p jno",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"h#10 h$! h$d h$l h$r h$o h$W h#32 h$, h$o h$l h$l h$e h$H >o o$ p jno",
            )],
        }],
    };

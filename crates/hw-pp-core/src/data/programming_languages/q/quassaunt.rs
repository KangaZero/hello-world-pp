//! Hello-world snippet for `ProgrammingLanguageName::Quassaunt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Quassaunt>
//! - <https://esolangs.org/wiki/Quassaunt>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Quassaunt => "Quassaunt",
    versions: [Quassaunt => "Quassaunt"],
    extensions: [],
}

const QUASSAUNT: &str = r"@@Hello       @@[Whitespace] %%Obtain      %%Head 
%%Concatenate @@world!       %%Concatenate %%Printtoconsole";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quassaunt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quassaunt,
            syntax: QUASSAUNT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUASSAUNT,
                    [(
                        TokenKind::Plain,
                        r"@@Hello       @@[Whitespace] %%Obtain      %%Head 
%%Concatenate @@world!       %%Concatenate %%Printtoconsole",
                    )],
                )
            },
        }],
    };

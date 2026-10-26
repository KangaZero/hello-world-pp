//! Hello-world snippet for `ProgrammingLanguageName::Digfill`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DigFill>
//! - <https://esolangs.org/wiki/DigFill>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Digfill => "DigFill",
    versions: [Digfill => "DigFill"],
    extensions: [],
}

const DIGFILL: &str = r"@n _Clear a space to output 1 bits_
&s&n&s&s&n&s&s&s _H_
&s&n&n&s&s&n&s&n _e_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&n&n&n&n _o_
&s&s&n&s&n&n&s&s _,_
&s&s&n&s&s&s&s&s _ _
&s&n&s&n&s&n&n&n _W_
&s&n&n&s&n&n&n&n _o_
&s&n&n&n&s&s&n&s _r_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&s&n&s&s _d_
&s&s&n&s&s&s&s&n _!_";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Digfill,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Digfill,
            syntax: DIGFILL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DIGFILL,
                    [(
                        TokenKind::Plain,
                        r"@n _Clear a space to output 1 bits_
&s&n&s&s&n&s&s&s _H_
&s&n&n&s&s&n&s&n _e_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&n&n&n&n _o_
&s&s&n&s&n&n&s&s _,_
&s&s&n&s&s&s&s&s _ _
&s&n&s&n&s&n&n&n _W_
&s&n&n&s&n&n&n&n _o_
&s&n&n&n&s&s&n&s _r_
&s&n&n&s&n&n&s&s _l_
&s&n&n&s&s&n&s&s _d_
&s&s&n&s&s&s&s&n _!_",
                    )],
                )
            },
        }],
    };

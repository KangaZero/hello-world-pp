//! Hello-world snippet for `ProgrammingLanguageName::NoWayQNoWayBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NO_WAY%3F_NO_WAY!>
//! - <https://esolangs.org/wiki/NO_WAY%3F_NO_WAY!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: NoWayQNoWayBang => "NO WAY? NO WAY!",
    versions: [NoWayQNoWayBang => "NO WAY? NO WAY!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NoWayQNoWayBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NoWayQNoWayBang,
            syntax: r"push 0
push 00100001
push 01100100
push 01101100
push 01110010
push 01101111
push 01110111
push 00100000
push 00101100
push 01101111
push 01101100
push 01101100
push 01100101
push 01001000
dup
que
if
  out 1
  jmp 1110
end
halt",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"push 0
push 00100001
push 01100100
push 01101100
push 01110010
push 01101111
push 01110111
push 00100000
push 00101100
push 01101111
push 01101100
push 01101100
push 01100101
push 01001000
dup
que
if
  out 1
  jmp 1110
end
halt",
            )],
        }],
    };

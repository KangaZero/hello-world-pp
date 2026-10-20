//! Hello-world snippet for `ProgrammingLanguageName::Ser2`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Ser2>
//! - <https://esolangs.org/wiki/Ser2>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ser2 => "Ser2",
    versions: [Ser2 => "Ser2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ser2,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ser2,
            syntax: r"!print--:#out:--:#c:#n:/wrote--:'@output--:#out: #c:#n:
!print--:#out:eol:     /wrote- :'@output--:#out:&0a:
!wrote--:'@iopair--:#out:#c:#n:/print--:#out:#n:
!wrote- :'@iopair--:#out:#c:   /#out:

!'@run-:#: / print--:#:
--:'H:--:'e:--:'l:--:'l:--:'o:--:',:--:' :
--:'w:--:'o:--:'r:--:'l:--:'d:--:'!:eol:",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"!print--:#out:--:#c:#n:/wrote--:'@output--:#out: #c:#n:
!print--:#out:eol:     /wrote- :'@output--:#out:&0a:
!wrote--:'@iopair--:#out:#c:#n:/print--:#out:#n:
!wrote- :'@iopair--:#out:#c:   /#out:

!'@run-:#: / print--:#:
--:'H:--:'e:--:'l:--:'l:--:'o:--:',:--:' :
--:'w:--:'o:--:'r:--:'l:--:'d:--:'!:eol:",
            )],
        }],
    };

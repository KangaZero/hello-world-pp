//! Hello-world snippet for `ProgrammingLanguageName::P0pcrn`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#P0pCrn>
//! - <https://esolangs.org/wiki/P0pCrn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: P0pcrn => "P0pCrn",
    versions: [P0pcrn => "P0pCrn"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::P0pcrn,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::P0pcrn,
            syntax: concat!(
                r#"_P    _0    _p    _!
@o    #o    $o    st
p_    p_    p_    r_
Th    ms    P0    op
is    g_    pC 
is    op    rn
wh    P0    =_
at    pC    He
co    rn    ll
mm          o_
en          wo
ts          rl
lo          d!
ok
li 
ke 
in
P0
pC
rn
!

Outputs ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"_P    _0    _p    _!
@o    #o    $o    st
p_    p_    p_    r_
Th    ms    P0    op
is    g_    pC 
is    op    rn
wh    P0    =_
at    pC    He
co    rn    ll
mm          o_
en          wo
ts          rl
lo          d!
ok
li 
ke 
in
P0
pC
rn
!

Outputs ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

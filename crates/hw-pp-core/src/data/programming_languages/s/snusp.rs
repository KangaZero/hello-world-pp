//! Hello-world snippet for `ProgrammingLanguageName::Snusp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SNUSP>
//! - <https://esolangs.org/wiki/SNUSP>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Snusp => "SNUSP",
    versions: [Snusp => "SNUSP"],
    extensions: [],
}

const SNUSP: &str = r"   /e+++++++++++++++++++++++++++++.\  
   ./\/\/\  /+++\!>.+++o.l.+++++++l/                  #/?\  
$H!\++++++\ +   \comma.------------ .<w++++++++.\ /?\<!\-/
   /++++++/ +/\                      /.--------o/ \-/!.++++++++++/?\n
 /=\++++++\ +\\!=++++++\             \r+++.l------.d--------.>+.!\-/
 \!\/\/\/\/ \++++++++++/";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snusp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snusp,
            syntax: SNUSP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SNUSP,
                    [(
                        TokenKind::Plain,
                        r"   /e+++++++++++++++++++++++++++++.\  
   ./\/\/\  /+++\!>.+++o.l.+++++++l/                  #/?\  
$H!\++++++\ +   \comma.------------ .<w++++++++.\ /?\<!\-/
   /++++++/ +/\                      /.--------o/ \-/!.++++++++++/?\n
 /=\++++++\ +\\!=++++++\             \r+++.l------.d--------.>+.!\-/
 \!\/\/\/\/ \++++++++++/",
                    )],
                )
            },
        }],
    };

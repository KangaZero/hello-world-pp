//! Hello-world snippet for `ProgrammingLanguageName::Flip`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Flip>
//! - <https://esolangs.org/wiki/Flip>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Flip => "Flip",
    versions: [Flip => "Flip"],
    extensions: [],
}

const FLIP: &str = r" >0> 5+  \
@'+  
 >0>4*+\
+,  /    /
vXv/   /
  \   \  \  \  \  \  \  \  \  \  \  \  \  \ 
      4@ 6@ 7@ 7@ 7@ 0@ 5@ 7@ 8@ 7@ 6@ 0@ 2@
    *<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<~<<
  P++<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<~<<
   Q  0@ 9@ 6@ 6@ 9@ 0@ 5@ 9@ 2@ 6@ 8@ 1@ 2@
\     /  /  /  /  /  /  /  /  /  /  /  /  /";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flip,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flip,
            syntax: FLIP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLIP,
                    [(
                        TokenKind::Plain,
                        r" >0> 5+  \
@'+  
 >0>4*+\
+,  /    /
vXv/   /
  \   \  \  \  \  \  \  \  \  \  \  \  \  \ 
      4@ 6@ 7@ 7@ 7@ 0@ 5@ 7@ 8@ 7@ 6@ 0@ 2@
    *<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<~<<
  P++<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<~<<
   Q  0@ 9@ 6@ 6@ 9@ 0@ 5@ 9@ 2@ 6@ 8@ 1@ 2@
\     /  /  /  /  /  /  /  /  /  /  /  /  /",
                    )],
                )
            },
        }],
    };

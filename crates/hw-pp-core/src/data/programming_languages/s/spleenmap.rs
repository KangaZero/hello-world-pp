//! Hello-world snippet for `ProgrammingLanguageName::Spleenmap`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Spleenmap>
//! - <https://esolangs.org/wiki/Spleenmap>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Spleenmap => "Spleenmap",
    versions: [Spleenmap => "Spleenmap"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spleenmap,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spleenmap,
            syntax: r"   .++  ...+...+...
 +++ +* .         +
++      ++       +.
.        +       +
.+       + +..+ ++
 ++ +++ .+ +  + +
  +.. + .  + .+ .++
      . ...+ +    +
    ++.      +. ++.
    +         . .
    .+       ++ ++
     +++     +   +++   ?++.
       +     +     +      .
       .     +   ++.     ++
      ++     ++. +  +++  +
      +        . +... ++ +
      .+   ++. .       +..
       +++ + +++
         ++.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"   .++  ...+...+...
 +++ +* .         +
++      ++       +.
.        +       +
.+       + +..+ ++
 ++ +++ .+ +  + +
  +.. + .  + .+ .++
      . ...+ +    +
    ++.      +. ++.
    +         . .
    .+       ++ ++
     +++     +   +++   ?++.
       +     +     +      .
       .     +   ++.     ++
      ++     ++. +  +++  +
      +        . +... ++ +
      .+   ++. .       +..
       +++ + +++
         ++.",
            )],
        }],
    };

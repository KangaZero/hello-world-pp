//! Hello-world snippet for `ProgrammingLanguageName::Iterate`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Iterate>
//! - <https://esolangs.org/wiki/Iterate>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Iterate => "Iterate",
    versions: [Iterate => "Iterate"],
    extensions: [],
}

const ITERATE: &str = r"(*)1<
  *1< (1*)72< *~n< &1 > ~@ ! > >      // H
  *1< (1*)101< *~n< &1 > ~@ ! > >     // e
  *1< (1*)108< *~n< &1 > ~@ ~@        // ll
    *1< (2*)111< *~n< &2 > ~@         // o
      *1< (3*)44< *~n< &3 > ~@ ! > >  // ,
      *1< (3*)32< *~n< &3 > ~@ ! > >  //  
      *1< (3*)119< *~n< &3 > ~@ ! > > // w
    ~@ ! > >                          // o
    *1< (2*)114< *~n< &2 > ~@ ! > >   // r
  ~@ ! > >                            // l
  *1< (1*)100< *~n< &1 > ~@ ! > >     // d
  *1< (1*)33< *~n< &1 > ~@ ! > >      // !
>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Iterate,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Iterate,
            syntax: ITERATE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ITERATE,
                    [(
                        TokenKind::Plain,
                        r"(*)1<
  *1< (1*)72< *~n< &1 > ~@ ! > >      // H
  *1< (1*)101< *~n< &1 > ~@ ! > >     // e
  *1< (1*)108< *~n< &1 > ~@ ~@        // ll
    *1< (2*)111< *~n< &2 > ~@         // o
      *1< (3*)44< *~n< &3 > ~@ ! > >  // ,
      *1< (3*)32< *~n< &3 > ~@ ! > >  //  
      *1< (3*)119< *~n< &3 > ~@ ! > > // w
    ~@ ! > >                          // o
    *1< (2*)114< *~n< &2 > ~@ ! > >   // r
  ~@ ! > >                            // l
  *1< (1*)100< *~n< &1 > ~@ ! > >     // d
  *1< (1*)33< *~n< &1 > ~@ ! > >      // !
>",
                    )],
                )
            },
        }],
    };

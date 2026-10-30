//! Hello-world snippet for `ProgrammingLanguageName::Sword`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sword>
//! - <https://esolangs.org/wiki/User:AnotherUser05%2FArchive%2FSword>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Sword => "Sword",
    versions: [Sword => "Sword"],
    extensions: [],
}

const SWORD: &str = r"-|> H
--|========>
--|===>
---|===========>
-|> e
--|==>
--|=>
--|==>
---|===========>
----|===> repeat 2 times
-|> l
--|==>
--|=>
--|=========>
---|===========>
--------|> end
-|> o
--|==>
--|==>
--|==>
---|===========>
-|> ,
--|=====>
--|=====>
---|===========>
-|> space
--|====>
--|===>
---|===========>
-|> W
--|=========>
--|========>
---|===========>
-|> o
--|==>
--|==>
--|==>
---|===========>
-|> r
--|==>
--|==>
--|=====>
---|===========>
-|> l
--|==>
--|=>
--|=========>
---|===========>
-|> d
--|==>
--|=>
--|=>
---|===========>
-|> !
--|====>
--|====>
---|===========>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sword,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sword,
            syntax: SWORD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SWORD,
                    [(
                        TokenKind::Plain,
                        r"-|> H
--|========>
--|===>
---|===========>
-|> e
--|==>
--|=>
--|==>
---|===========>
----|===> repeat 2 times
-|> l
--|==>
--|=>
--|=========>
---|===========>
--------|> end
-|> o
--|==>
--|==>
--|==>
---|===========>
-|> ,
--|=====>
--|=====>
---|===========>
-|> space
--|====>
--|===>
---|===========>
-|> W
--|=========>
--|========>
---|===========>
-|> o
--|==>
--|==>
--|==>
---|===========>
-|> r
--|==>
--|==>
--|=====>
---|===========>
-|> l
--|==>
--|=>
--|=========>
---|===========>
-|> d
--|==>
--|=>
--|=>
---|===========>
-|> !
--|====>
--|====>
---|===========>",
                    )],
                )
            },
        }],
    };

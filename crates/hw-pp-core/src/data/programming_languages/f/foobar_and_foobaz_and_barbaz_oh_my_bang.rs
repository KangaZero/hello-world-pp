//! Hello-world snippet for `ProgrammingLanguageName::FoobarAndFoobazAndBarbazOhMyBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Foobar_and_Foobaz_and_Barbaz,_oh_my!>
//! - <https://esolangs.org/wiki/Foobar_and_Foobaz_and_Barbaz,_oh_my!>

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: FoobarAndFoobazAndBarbazOhMyBang => "Foobar and Foobaz and Barbaz, oh my!",
    versions: [FoobarAndFoobazAndBarbazOhMyBang => "Foobar and Foobaz and Barbaz, oh my!"],
    extensions: [],
}

const FOOBAR_AND_FOOBAZ_AND_BARBAZ_OH_MY_BANG: &str = r"72 and 27 and 27, oh my.
101 and 101 and 101, oh my.
108 and 108 and 108, oh my.
108 and 108 and 108, oh my.
111 and 111 and 111, oh my.
44 and 44 and 44, oh my.
32 and 32 and 32, oh my.
119 and 119 and 119, oh my.
111 and 111 and 111, oh my.
114 and 114 and 114, oh my.
108 and 108 and 108, oh my.
100 and 100 and 100, oh my.
33 and 33 and 33, oh my.
10 and 10 and 10, oh my.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FoobarAndFoobazAndBarbazOhMyBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FoobarAndFoobazAndBarbazOhMyBang,
            syntax: FOOBAR_AND_FOOBAZ_AND_BARBAZ_OH_MY_BANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOOBAR_AND_FOOBAZ_AND_BARBAZ_OH_MY_BANG,
                    [(
                        TokenKind::Plain,
                        r"72 and 27 and 27, oh my.
101 and 101 and 101, oh my.
108 and 108 and 108, oh my.
108 and 108 and 108, oh my.
111 and 111 and 111, oh my.
44 and 44 and 44, oh my.
32 and 32 and 32, oh my.
119 and 119 and 119, oh my.
111 and 111 and 111, oh my.
114 and 114 and 114, oh my.
108 and 108 and 108, oh my.
100 and 100 and 100, oh my.
33 and 33 and 33, oh my.
10 and 10 and 10, oh my.",
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Pi`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pi>
//! - <https://esolangs.org/wiki/Pi>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pi => "Pi",
    versions: [Pi => "Pi"],
    extensions: [],
}

const PI: &str = r"  3.1415926535897932384226433232725028841271693992751358239749245923072164062
  822089986780348053431120629821483865332223366470908446395535832317223594083
  284831172502841037019385311052596446029489529303219642288009756259134461214
  751482337817834652812009021456285664234613486134553226482133236073603491413
  737242870066363155841744815239239622292545917053643278925902605113301301488
  204625213821429519415316024320522703627595939530921261373219226137931021185
  580744023794627495273518257527348912274384830113491298346743644406563430865
  2133494633522473749030217386093370277033921317659317670238267581846066440";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pi,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pi,
            syntax: PI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PI,
                    [(
                        TokenKind::Plain,
                        r"  3.1415926535897932384226433232725028841271693992751358239749245923072164062
  822089986780348053431120629821483865332223366470908446395535832317223594083
  284831172502841037019385311052596446029489529303219642288009756259134461214
  751482337817834652812009021456285664234613486134553226482133236073603491413
  737242870066363155841744815239239622292545917053643278925902605113301301488
  204625213821429519415316024320522703627595939530921261373219226137931021185
  580744023794627495273518257527348912274384830113491298346743644406563430865
  2133494633522473749030217386093370277033921317659317670238267581846066440",
                    )],
                )
            },
        }],
    };

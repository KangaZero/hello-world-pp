//! Hello-world snippet for `ProgrammingLanguageName::PizzaDelivery`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pizza_Delivery>
//! - <https://esolangs.org/wiki/Pizza_Delivery>

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
    language: PizzaDelivery => "Pizza Delivery",
    versions: [PizzaDelivery => "Pizza Delivery"],
    extensions: [],
}

const PIZZA_DELIVERY: &str =
    r"21d64d72d57d20d2Cd6Fd6Cd65d48`a`a``a`a`a`a`ddd`aaaa`ddddd`aaaaaa`a`.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PizzaDelivery,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PizzaDelivery,
            syntax: PIZZA_DELIVERY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PIZZA_DELIVERY,
                    [(
                        TokenKind::Plain,
                        r"21d64d72d57d20d2Cd6Fd6Cd65d48`a`a``a`a`a`a`ddd`aaaa`ddddd`aaaaaa`a`.",
                    )],
                )
            },
        }],
    };

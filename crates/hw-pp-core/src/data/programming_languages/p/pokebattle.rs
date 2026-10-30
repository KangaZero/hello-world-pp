//! Hello-world snippet for `ProgrammingLanguageName::Pokebattle`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pok%C3%A9battle>
//! - <https://esolangs.org/wiki/Pok%C3%A9Battle>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Pokebattle => "Pokébattle",
    versions: [Pokebattle => "Pokébattle"],
    extensions: [],
}

const POKEBATTLE: &str = "Trainer 1: Alice\n Togepi\n Emolga\n Talonflame\n Rhyperior\n Abra\n Pikachu\nTrainer 2: Bob\n Yveltal\n Raichu\n Oranguru\n Nidoqueen\n Incineroar\n Avalugg \n\u{200c}\nBattle Start!\nTurn 0:\n Alice: Rhyperior Go!\n Bob: Avalugg Go!\n\nTurn 1:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 2:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 3:\n Alice: That's enough! Go Pikachu!\n Avalugg uses Splash!\n\nTurn 4:\n Pikachu uses Absorb!\n Bob: That's enough! Go Raichu!\n\nTurn 5:\n Alice: That's enough! Go Rhyperior!\n Raichu uses Absorb!\n\nTurn 6:\n Rhyperior uses Splash!\n Bob: That's enough! Go Avalugg!\n\nTurn 7:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 8:\n Alice uses Max Potion!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 9:\n Rhyperior uses Flamethrower!\n Avalugg uses Splash!\n\nTurn 10:\n Rhyperior uses Swords Dance!\n Avalugg uses Swords Dance!\n\nTurn 11:\n Rhyperior uses Tackle! It's not very effective.\n Bob uses Max Potion!\n\nTurn 12:\n Rhyperior uses Skill Swap!\n Avalugg uses Swords Dance!\n\nTurn 13:\n Rhyperior uses Swords Dance!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 14:\n Alice uses Hyper Potion!\n Bob: That's enough! Go Raichu!\n\nTurn 15:\n Rhyperior uses Swords Dance!\n Bob uses Super Potion!\n\nTurn 16:\n Alice: That's enough! Go Pikachu!\n Raichu uses Tackle! It's not very effective.\n\nTurn 17:\n Pikachu uses Swords Dance!\n Raichu uses Swords Dance!\n\nTurn 18:\n Alice: That's enough! Go Rhyperior!\n Bob uses Potion!\n\nTurn 19:\n Alice uses Max Potion!\n Bob: That's enough! Go Avalugg!\n\nTurn 20:\n Rhyperior uses Swords Dance!\n Avalugg uses Flamethrower! It's super effective!\n\nTurn 21:\n Rhyperior uses Swords Dance!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 22:\n Alice uses Hyper Potion!\n Avalugg uses Splash!\n\nTurn 23:\n Rhyperior uses Swords Dance!\n Avalugg uses Swords Dance!\n\nTurn 24:\n Rhyperior uses Flamethrower! It's super effective!\n Avalugg uses Swords Dance!\n\nTurn 25:\n Alice: That's enough! Go Abra!\n Bob: That's enough! Go Raichu!\n\nTurn 26:\n Abra uses Splash!\n Raichu uses Swords Dance!\n\nBattle End!\nWinner: Alice";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pokebattle,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pokebattle,
            syntax: POKEBATTLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POKEBATTLE,
                    [(
                        TokenKind::Plain,
                        "Trainer 1: Alice\n Togepi\n Emolga\n Talonflame\n Rhyperior\n Abra\n Pikachu\nTrainer 2: Bob\n Yveltal\n Raichu\n Oranguru\n Nidoqueen\n Incineroar\n Avalugg \n\u{200c}\nBattle Start!\nTurn 0:\n Alice: Rhyperior Go!\n Bob: Avalugg Go!\n\nTurn 1:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 2:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 3:\n Alice: That's enough! Go Pikachu!\n Avalugg uses Splash!\n\nTurn 4:\n Pikachu uses Absorb!\n Bob: That's enough! Go Raichu!\n\nTurn 5:\n Alice: That's enough! Go Rhyperior!\n Raichu uses Absorb!\n\nTurn 6:\n Rhyperior uses Splash!\n Bob: That's enough! Go Avalugg!\n\nTurn 7:\n Rhyperior uses Absorb!\n Avalugg uses Absorb!\n\nTurn 8:\n Alice uses Max Potion!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 9:\n Rhyperior uses Flamethrower!\n Avalugg uses Splash!\n\nTurn 10:\n Rhyperior uses Swords Dance!\n Avalugg uses Swords Dance!\n\nTurn 11:\n Rhyperior uses Tackle! It's not very effective.\n Bob uses Max Potion!\n\nTurn 12:\n Rhyperior uses Skill Swap!\n Avalugg uses Swords Dance!\n\nTurn 13:\n Rhyperior uses Swords Dance!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 14:\n Alice uses Hyper Potion!\n Bob: That's enough! Go Raichu!\n\nTurn 15:\n Rhyperior uses Swords Dance!\n Bob uses Super Potion!\n\nTurn 16:\n Alice: That's enough! Go Pikachu!\n Raichu uses Tackle! It's not very effective.\n\nTurn 17:\n Pikachu uses Swords Dance!\n Raichu uses Swords Dance!\n\nTurn 18:\n Alice: That's enough! Go Rhyperior!\n Bob uses Potion!\n\nTurn 19:\n Alice uses Max Potion!\n Bob: That's enough! Go Avalugg!\n\nTurn 20:\n Rhyperior uses Swords Dance!\n Avalugg uses Flamethrower! It's super effective!\n\nTurn 21:\n Rhyperior uses Swords Dance!\n Avalugg uses Tackle! It's not very effective.\n\nTurn 22:\n Alice uses Hyper Potion!\n Avalugg uses Splash!\n\nTurn 23:\n Rhyperior uses Swords Dance!\n Avalugg uses Swords Dance!\n\nTurn 24:\n Rhyperior uses Flamethrower! It's super effective!\n Avalugg uses Swords Dance!\n\nTurn 25:\n Alice: That's enough! Go Abra!\n Bob: That's enough! Go Raichu!\n\nTurn 26:\n Abra uses Splash!\n Raichu uses Swords Dance!\n\nBattle End!\nWinner: Alice",
                    )],
                )
            },
        }],
    };

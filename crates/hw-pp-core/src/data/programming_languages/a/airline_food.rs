//! Hello-world snippet for `ProgrammingLanguageName::AirlineFood`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Airline_food>
//! - <https://esolangs.org/wiki/Airline_Food>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AirlineFood => "Airline food",
    versions: [AirlineFood => "Airline food"],
    extensions: [],
}

const AIRLINE_FOOD: &str = r"You ever notice this?
What's the deal with this thing? It's kinda like this.
What's the deal with airline food?
What's the deal with it?
It's kinda like this thing. Just like it. 
Let's talk about this thing. It's kinda like this thing. It's kinda like this.
Yeah, Just like this thing. Just like it. Not like this.
What's the deal with pilots? Just like it. Not like this. Just like it. See?
Let's talk about this thing. Not like this.
What's the deal with baggage claim? Just like this thing. Just like it. It's kinda like pilots.
What's the deal with luggage? Just like baggage claim. It's kinda like this. It's kinda like this. Not like it. See?
Let's talk about baggage claim. See? See? It's kinda like this thing. Not like this. See?
Let's talk about it. Um, See? It's kinda like this. Not like this thing. Not like it. See?
Let's talk about baggage claim. It's kinda like it. Not like this. See? It's kinda like this. Not like it. See? It's kinda like this thing. Not like this. 
See? Not like this thing. Not like this. Not like this. See?
Let's talk about luggage. Not like this. See?
Let's talk about it. Um, It's kinda like this. See?";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AirlineFood,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AirlineFood,
            syntax: AIRLINE_FOOD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AIRLINE_FOOD,
                    [(
                        TokenKind::Plain,
                        r"You ever notice this?
What's the deal with this thing? It's kinda like this.
What's the deal with airline food?
What's the deal with it?
It's kinda like this thing. Just like it. 
Let's talk about this thing. It's kinda like this thing. It's kinda like this.
Yeah, Just like this thing. Just like it. Not like this.
What's the deal with pilots? Just like it. Not like this. Just like it. See?
Let's talk about this thing. Not like this.
What's the deal with baggage claim? Just like this thing. Just like it. It's kinda like pilots.
What's the deal with luggage? Just like baggage claim. It's kinda like this. It's kinda like this. Not like it. See?
Let's talk about baggage claim. See? See? It's kinda like this thing. Not like this. See?
Let's talk about it. Um, See? It's kinda like this. Not like this thing. Not like it. See?
Let's talk about baggage claim. It's kinda like it. Not like this. See? It's kinda like this. Not like it. See? It's kinda like this thing. Not like this. 
See? Not like this thing. Not like this. Not like this. See?
Let's talk about luggage. Not like this. See?
Let's talk about it. Um, It's kinda like this. See?",
                    )],
                )
            },
        }],
    };

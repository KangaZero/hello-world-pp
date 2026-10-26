//! Hello-world snippet for `ProgrammingLanguageName::Oscolang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#OSColang>
//! - <https://esolangs.org/wiki/OSColang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Oscolang => "OSColang",
    versions: [Oscolang => "OSColang"],
    extensions: [],
}

const OSCOLANG: &str = r"BFDI 1a: Take the Plunge 
II 1: The Crappy Cliff 
BFDI:TPOT 1: You Know Those Buttons Don't Do Anything, Right? 
BFB 3: Why Would You Do This on a Swingset 
BFDIE 3: Everyone’s Enemies 
BFAM 4: Hello Again... 
BFDI 4: Sweet Tooth 
BFB 1: Getting Teardrop to Talk 
BFDI 1b: Take the Plunge 
AIB 2: Smarter & Boulder 
BFDI:TPOT 2: The Worst Day of Black Hole's Life 
TAW 1a: Who, Me? 
BFDIA 3: Insectophobe's Nightmare 3 
BFDI 3: Are You Smarter Than a Snowball? 
BFDIA 1: Yeah, Who? I Wanna Know 
BFDI 5: Bridge Crossing 
BFB 5: Fortunate Ben 
BFDIA 5a: Get in the Van 
BFDI 2: Barriers and Pitfalls 
BFDI:TPOT 3: Getting Puffball To Think About Rollercoasters 
BFDIA 5c: No More Snow! 
II 3: The Arena of Death 
II2 3: Tri Your Best 
BFDI 12: A Leg Up in the Race 
AIB 5: Sun Blocked 
II 2: A Lemony Lesson 
BFDIA 5d: It's a Monster 
II2 2: Marsh on Mars 
BFDI:TPOT 4: Gardening Zero 
BFDIA 5e: The Long-lost Yoyle City 
III 3: Snapshot Showdown 
IIIV 3: Run the Risk! 
TDOS:END 2: All the Time in the World 
BFB 4: Today's Very Special Episode 
BFDIA 6: Well Rested 
TAW 1b: Yes, You! 
III 2: The Shame of the Name 
BFDI:TPOT 5: Fishes and Dishes 
BFDI 19: Rescission 
TDOS 3: Tower of Evil 
AIB 3: Get A Grip 
BFDI 10: Crybaby! 
II 5: The Stacker 
BFDIA 8: Meaty 
BFDI 20: Gardening Hero 
IIIV 2: Cob Mentality 
BFDI:TPOT 6: The Great Goikian Bake-Off 
BFDI 21: The Glistening 
BFB 6: Four Goes Too Far 
BFDI:TPOT 7: The Seven Wonders of Goiky 
BFB 30: Chapter Complete ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oscolang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oscolang,
            syntax: OSCOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OSCOLANG,
                    [(
                        TokenKind::Plain,
                        r"BFDI 1a: Take the Plunge 
II 1: The Crappy Cliff 
BFDI:TPOT 1: You Know Those Buttons Don't Do Anything, Right? 
BFB 3: Why Would You Do This on a Swingset 
BFDIE 3: Everyone’s Enemies 
BFAM 4: Hello Again... 
BFDI 4: Sweet Tooth 
BFB 1: Getting Teardrop to Talk 
BFDI 1b: Take the Plunge 
AIB 2: Smarter & Boulder 
BFDI:TPOT 2: The Worst Day of Black Hole's Life 
TAW 1a: Who, Me? 
BFDIA 3: Insectophobe's Nightmare 3 
BFDI 3: Are You Smarter Than a Snowball? 
BFDIA 1: Yeah, Who? I Wanna Know 
BFDI 5: Bridge Crossing 
BFB 5: Fortunate Ben 
BFDIA 5a: Get in the Van 
BFDI 2: Barriers and Pitfalls 
BFDI:TPOT 3: Getting Puffball To Think About Rollercoasters 
BFDIA 5c: No More Snow! 
II 3: The Arena of Death 
II2 3: Tri Your Best 
BFDI 12: A Leg Up in the Race 
AIB 5: Sun Blocked 
II 2: A Lemony Lesson 
BFDIA 5d: It's a Monster 
II2 2: Marsh on Mars 
BFDI:TPOT 4: Gardening Zero 
BFDIA 5e: The Long-lost Yoyle City 
III 3: Snapshot Showdown 
IIIV 3: Run the Risk! 
TDOS:END 2: All the Time in the World 
BFB 4: Today's Very Special Episode 
BFDIA 6: Well Rested 
TAW 1b: Yes, You! 
III 2: The Shame of the Name 
BFDI:TPOT 5: Fishes and Dishes 
BFDI 19: Rescission 
TDOS 3: Tower of Evil 
AIB 3: Get A Grip 
BFDI 10: Crybaby! 
II 5: The Stacker 
BFDIA 8: Meaty 
BFDI 20: Gardening Hero 
IIIV 2: Cob Mentality 
BFDI:TPOT 6: The Great Goikian Bake-Off 
BFDI 21: The Glistening 
BFB 6: Four Goes Too Far 
BFDI:TPOT 7: The Seven Wonders of Goiky 
BFB 30: Chapter Complete ",
                    )],
                )
            },
        }],
    };

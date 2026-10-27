//! Hello-world snippet for `ProgrammingLanguageName::Cratefuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cratefuck>
//! - <https://esolangs.org/wiki/Cratefuck>

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
    language: Cratefuck => "Cratefuck",
    versions: [Cratefuck => "Cratefuck"],
    extensions: [],
}

const CRATEFUCK: &str = r#"*[>*<*>>*<<*>>>*<<<*>>>>*<<<<*] Make 4 stacks of 64 in rooms 1, 2, 3, & 4; (0, 64, 64, 64, 64
>>>>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*. Manually move 8 crates from room 4 to room 3 and print "H"; (0, 64, 64, 72, 56
>*[>*<*>>*<<*] Split the remaining 56 crates in room 4 into 2 stacks of 28 in rooms 5 & 6; (0, 64, 64, 72, 0, 28, 28
>>*[<<<*>>>*] Move all 28 crates from room 6 into room 3; (0, 64, 64, 100, 0, 28, 0
<*<<*. Move 1 crate from room 5 to room 3 and print "e"; (0, 64, 64, 101, 0, 27, 0
<*>*<*>*<*>*<*>*<*>*<*>*<*>*.. Move 7 crates from room 2 to room 3 and print "ll"; (0, 64, 57, 108, 0, 27, 0
<*>*<*>*<*>*. Move 3 more crates from room 2 to room 3 and print "o"; (0, 64, 54, 111, 0, 27, 0
<*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>. Move 10 crates from room 2 to room 1 and print ","; (0, 74, 44, 111, 0, 27, 0
>*>>*<<*>>*<<*>>*<<*>>*<<*>>*. Move 5 crates from room 3 to room 5 and print " "; (0, 74, 44, 106, 0, 32, 0
<<<<*[>*<*]>>*<*. Move 74 crates from room 1 and 1 crate from room 3 to room 2 and print "w"; (0, 0, 119, 105, 0, 32, 0
*>*<*>*<*>*<*>*<*>*<*>*. Move 6 crates from room 2 to room 3 and print "o"; (0, 0, 113, 111, 0, 32, 0
*<*. Move 1 crate from room 3 to room 2 and print "r"; (0, 0, 114, 110, 0, 32, 0
>*<*>*<*>. Move 2 crates from room 3 to room 2 and print "l"; (0, 0, 116, 108, 0, 32, 0
*<*>*<*>*<*>*<*>*<*>*<*>*<*>*. Move 7 crates from room 3 to room 2, grab an 8th crate from room 3, and print "d"; (0, 0, 123, 100, 0, 32, 0
>>*. Move the 8th crate to room 5 and print "!"; (0, 0, 123, 100, 0, 33, 0"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cratefuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cratefuck,
            syntax: CRATEFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CRATEFUCK,
                    [(
                        TokenKind::Plain,
                        r#"*[>*<*>>*<<*>>>*<<<*>>>>*<<<<*] Make 4 stacks of 64 in rooms 1, 2, 3, & 4; (0, 64, 64, 64, 64
>>>>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*. Manually move 8 crates from room 4 to room 3 and print "H"; (0, 64, 64, 72, 56
>*[>*<*>>*<<*] Split the remaining 56 crates in room 4 into 2 stacks of 28 in rooms 5 & 6; (0, 64, 64, 72, 0, 28, 28
>>*[<<<*>>>*] Move all 28 crates from room 6 into room 3; (0, 64, 64, 100, 0, 28, 0
<*<<*. Move 1 crate from room 5 to room 3 and print "e"; (0, 64, 64, 101, 0, 27, 0
<*>*<*>*<*>*<*>*<*>*<*>*<*>*.. Move 7 crates from room 2 to room 3 and print "ll"; (0, 64, 57, 108, 0, 27, 0
<*>*<*>*<*>*. Move 3 more crates from room 2 to room 3 and print "o"; (0, 64, 54, 111, 0, 27, 0
<*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>*<*>. Move 10 crates from room 2 to room 1 and print ","; (0, 74, 44, 111, 0, 27, 0
>*>>*<<*>>*<<*>>*<<*>>*<<*>>*. Move 5 crates from room 3 to room 5 and print " "; (0, 74, 44, 106, 0, 32, 0
<<<<*[>*<*]>>*<*. Move 74 crates from room 1 and 1 crate from room 3 to room 2 and print "w"; (0, 0, 119, 105, 0, 32, 0
*>*<*>*<*>*<*>*<*>*<*>*. Move 6 crates from room 2 to room 3 and print "o"; (0, 0, 113, 111, 0, 32, 0
*<*. Move 1 crate from room 3 to room 2 and print "r"; (0, 0, 114, 110, 0, 32, 0
>*<*>*<*>. Move 2 crates from room 3 to room 2 and print "l"; (0, 0, 116, 108, 0, 32, 0
*<*>*<*>*<*>*<*>*<*>*<*>*<*>*. Move 7 crates from room 3 to room 2, grab an 8th crate from room 3, and print "d"; (0, 0, 123, 100, 0, 32, 0
>>*. Move the 8th crate to room 5 and print "!"; (0, 0, 123, 100, 0, 33, 0"#,
                    )],
                )
            },
        }],
    };

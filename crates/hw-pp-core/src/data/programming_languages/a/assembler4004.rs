//! Hello-world snippet for `ProgrammingLanguageName::Assembler4004`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 4004.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%204004.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler4004 => "Assembler 4004",
    versions: [Assembler4004 => "Assembler 4004"],
    extensions: [],
}

const ASSEMBLER4004: &str = r"; H  e  l  l  o     W  o  r  l  d
; 48 65 6C 6C 6F 20 57 6F 72 6C 64
START
        LDM $4          ; High nibble of letter 'H'
        WRR             ; Write to ROM output port
        LDM $8          ; Low nibble of letter 'H'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'e'
        WRR             ; Write to ROM output port
        LDM $5          ; Low nibble of letter 'e'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'l'
        WRR             ; Write to ROM output port
        LDM $C          ; Low nibble of letter 'l'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'l'
        WRR             ; Write to ROM output port
        LDM $C          ; Low nibble of letter 'l'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'o'
        WRR             ; Write to ROM output port
        LDM $F          ; Low nibble of letter 'o'
        WRR             ; Write to ROM output port

        LDM $2          ; High nibble of 'space'
        WRR             ; Write to ROM output port
        LDM $0          ; Low nibble of 'space'
        WRR             ; Write to ROM output port


        LDM $5          ; High nibble of letter 'W'
        WRR             ; Write to ROM output port
        LDM $7          ; Low nibble of letter 'W'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'o'
        WRR             ; Write to ROM output port
        LDM $F          ; Low nibble of letter 'o'
        WRR             ; Write to ROM output port

        LDM $7          ; High nibble of letter 'r'
        WRR             ; Write to ROM output port
        LDM $2          ; Low nibble of letter 'r'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'l'
        WRR             ; Write to ROM output port
        LDM $C          ; Low nibble of letter 'l'
        WRR             ; Write to ROM output port

        LDM $6          ; High nibble of letter 'd'
        WRR             ; Write to ROM output port
        LDM $4          ; Low nibble of letter 'd'
        WRR             ; Write to ROM output port
END
        JUN END";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler4004,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler4004,
            syntax: ASSEMBLER4004,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER4004,
                    [
                        (TokenKind::Comment, r"; H  e  l  l  o     W  o  r  l  d"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; 48 65 6C 6C 6F 20 57 6F 72 6C 64"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"START"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'H'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"8"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'H'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'e'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"5"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'e'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $C          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $C          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'o'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $F          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'o'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of 'space'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of 'space'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"


        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"5"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'W'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"7"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'W'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'o'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $F          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'o'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"7"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'r'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'r'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $C          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'l'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; High nibble of letter 'd'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"LDM"),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Plain, r"          "),
                        (TokenKind::Comment, r"; Low nibble of letter 'd'"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"WRR"),
                        (TokenKind::Plain, r"             "),
                        (TokenKind::Comment, r"; Write to ROM output port"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"JUN"),
                        (TokenKind::Plain, r" END"),
                    ],
                )
            },
        }],
    };

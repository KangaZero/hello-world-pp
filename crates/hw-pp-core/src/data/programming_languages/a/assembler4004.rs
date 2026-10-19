//! Hello-world snippet for `ProgrammingLanguageName::Assembler4004`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 4004.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Assembler4004 => "Assembler 4004",
    versions: [Assembler4004 => "Assembler 4004"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler4004,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler4004,
            syntax: r"; H  e  l  l  o     W  o  r  l  d
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
        JUN END",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"; H  e  l  l  o     W  o  r  l  d
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
        JUN END",
            )],
        }],
    };

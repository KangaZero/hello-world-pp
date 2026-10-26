//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Console`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(Z80&nbsp;Console).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28Z80%26nbsp%3BConsole%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(Z80%C2%A0Console)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerZ80Console => "Assembler (Z80 Console)",
    versions: [AssemblerZ80Console => "Assembler (Z80 Console)"],
    extensions: [],
}

const ASSEMBLER_Z80_CONSOLE: &str = concat!(
    r#"; This is a ""#,
    output_for_hello_world!(),
    r#"" program for Z80 and TMS9918 / TMS9928 / TMS9929 /
; V9938 or V9958 VDP.
; That means that this should work on SVI, MSX, Colecovision, Memotech,
; and many other Z80 based home computers or game consoles.
;
; Because we don't know what system is used, we don't know where RAM
; is, so we can't use stack in this program.
;
; This version of "#,
    output_for_hello_world!(),
    r#" was written by Timo "NYYRIKKI" Soilamaa
; 17.10.2001
;
;----------------------------------------------------------------------
; Configure this part:

DATAP: EQU #98 ; VDP Data port #98 works on all MSX models
; (TMS9918/TMS9929/V9938 or V9958)
; #80 works on SVI 
; (for other platforms you have to figure this out by your self)

CMDP: EQU #99 ; VDP Command port #99 works on all MSX models
; (TMS9918/TMS9929/V9938 or V9958)
; #81 works on SVI
; (for other platforms you have to figure this out by your self)
;-----------------------------------------------------------------------
; Program starts here:

ORG 0 ; Z80 starts always from here when power is turned on
DI ; We don't know, how interrupts works in this system, so we disable them.

; Let's set VDP write address to #0000
XOR A
OUT (CMDP),A
LD A,#40
OUT (CMDP),A

; Now let's clear first 16Kb of VDP memory
LD B,0
LD HL,#3FFF
LD C,DATAP
CLEAR:
OUT (C),B
DEC HL
LD A,H
OR L
NOP ; Let's wait 8 clock cycles just in case VDP is not quick enough.
NOP
JR NZ,CLEAR

; Now it is time to set up VDP registers:
;----------------------------------------
; Register 0 to #0
;
; Set mode selection bit M3 (maybe also M4 & M5) to zero and 
; disable external video & horizontal interrupt
LD C,CMDP
LD E,#80

OUT (C),A
OUT (C),E
;---------------------------------------- 
; Register 1 to #50
;
; Select 40 column mode, enable screen and disable vertical interrupt

LD A,#50
INC E
OUT (C),A
OUT (C),E
;---------------------------------------- 
; Register 2 to #0
;
; Set pattern name table to #0000

XOR A
INC E
OUT (C),A
OUT (C),E
;---------------------------------------- 
; Register 3 is ignored as 40 column mode does not need color table
;
INC E
;---------------------------------------- 
; Register 4 to #1
; Set pattern generator table to #800

INC A
INC E

OUT (C),A
OUT (C),E
;---------------------------------------- 
; Registers 5 (Sprite attribute) & 6 (Sprite pattern) are ignored 
; as 40 column mode does not have sprites

INC E
INC E
;---------------------------------------- 
; Register 7 to #F0
; Set colors to white on black

LD A,#F0
INC E
OUT (C),A
OUT (C),E
;----------------------------------------

; Let's set VDP write address to #808 so, that we can write
; character set to memory
; (No need to write SPACE it is clear char already)
LD A,8
OUT (C),A
LD A,#48
OUT (C),A

; Let's copy character set
LD HL,CHARS
LD B, CHARS_END-CHARS
COPYCHARS:
LD A,(HL)
OUT (DATAP),A
INC HL
NOP ; Let's wait 8 clock cycles just in case VDP is not quick enough.
NOP
DJNZ COPYCHARS

; Let's set write address to start of name table
XOR A
OUT (C),A
LD A,#40
OUT (C),A

; Let's put characters to screen
LD HL,ORDER
LD B,ORDER_END-ORDER
COPYORDER:
LD A,(HL)
OUT (DATAP),A
INC HL

JR OVERNMI
NOP
NOP

; Here is address #66, that is entry for NMI
RETN ;Return from NMI

OVERNMI:
DJNZ COPYORDER

; The end
HALT

; Character set:
; --------------
ORDER:
DEFB 1,2,3,3,4,0,5,4,6,3,7
ORDER_END:

CHARS:

; H
DEFB %10001000
DEFB %10001000
DEFB %10001000
DEFB %11111000
DEFB %10001000
DEFB %10001000
DEFB %10001000
DEFB %00000000
; e
DEFB %00000000
DEFB %00000000
DEFB %01110000
DEFB %10001000
DEFB %11111000
DEFB %10000000
DEFB %01110000
DEFB %00000000
; l
DEFB %01100000
DEFB %00100000
DEFB %00100000
DEFB %00100000
DEFB %00100000
DEFB %00100000
DEFB %01110000
DEFB %00000000
; o
DEFB %00000000
DEFB %00000000
DEFB %01110000
DEFB %10001000
DEFB %10001000
DEFB %10001000
DEFB %01110000
DEFB %00000000
; W
DEFB %10001000
DEFB %10001000
DEFB %10001000
DEFB %10101000
DEFB %10101000
DEFB %11011000
DEFB %10001000
DEFB %00000000

; r
DEFB %00000000
DEFB %00000000
DEFB %10110000
DEFB %11001000
DEFB %10000000
DEFB %10000000
DEFB %10000000
DEFB %00000000
; d
DEFB %00001000
DEFB %00001000
DEFB %01101000
DEFB %10011000
DEFB %10001000
DEFB %10011000
DEFB %01101000
DEFB %00000000
chars_end:"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Console,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Console,
            syntax: ASSEMBLER_Z80_CONSOLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_Z80_CONSOLE,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"; This is a ""#,
                                output_for_hello_world!(),
                                r#"" program for Z80 and TMS9918 / TMS9928 / TMS9929 /"#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; V9938 or V9958 VDP."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; That means that this should work on SVI, MSX, Colecovision, Memotech,",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; and many other Z80 based home computers or game consoles.",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Because we don't know what system is used, we don't know where RAM",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; is, so we can't use stack in this program.",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; This version of ",
                                output_for_hello_world!(),
                                r#" was written by Timo "NYYRIKKI" Soilamaa"#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; 17.10.2001"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";----------------------------------------------------------------------",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Configure this part:"),
                        (
                            TokenKind::Plain,
                            r"

DATAP",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"EQU"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"98"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; VDP Data port #98 works on all MSX models",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; (TMS9918/TMS9929/V9938 or V9958)"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; #80 works on SVI "),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; (for other platforms you have to figure this out by your self)",
                        ),
                        (
                            TokenKind::Plain,
                            r"

CMDP",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"EQU"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"99"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; VDP Command port #99 works on all MSX models",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; (TMS9918/TMS9929/V9938 or V9958)"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; #81 works on SVI"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; (for other platforms you have to figure this out by your self)",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";-----------------------------------------------------------------------",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Program starts here:"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"ORG"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; Z80 starts always from here when power is turned on",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DI"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; We don't know, how interrupts works in this system, so we disable them.",
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Let's set VDP write address to #0000",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"XOR"),
                        (
                            TokenKind::Plain,
                            r" A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"CMDP"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"40"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"CMDP"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Now let's clear first 16Kb of VDP memory",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" B"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" HL"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"#3FFF
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" C"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"DATAP
CLEAR",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"B
",
                        ),
                        (TokenKind::Keyword, r"DEC"),
                        (
                            TokenKind::Plain,
                            r" HL
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"H
",
                        ),
                        (TokenKind::Keyword, r"OR"),
                        (
                            TokenKind::Plain,
                            r" L
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; Let's wait 8 clock cycles just in case VDP is not quick enough.",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"JR"),
                        (TokenKind::Plain, r" NZ"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"CLEAR

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Now it is time to set up VDP registers:",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";----------------------------------------",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Register 0 to #0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Set mode selection bit M3 (maybe also M4 & M5) to zero and ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; disable external video & horizontal interrupt",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" C"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"CMDP
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" E"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"80"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Register 1 to #50"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Select 40 column mode, enable screen and disable vertical interrupt",
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"50"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Register 2 to #0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Set pattern name table to #0000"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"XOR"),
                        (
                            TokenKind::Plain,
                            r" A
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Register 3 is ignored as 40 column mode does not need color table",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Register 4 to #1"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Set pattern generator table to #800"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" A
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E

",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Registers 5 (Sprite attribute) & 6 (Sprite pattern) are ignored ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; as 40 column mode does not have sprites",
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";---------------------------------------- ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Register 7 to #F0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; Set colors to white on black"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"#F0
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" E
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"E
",
                        ),
                        (
                            TokenKind::Comment,
                            r";----------------------------------------",
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Let's set VDP write address to #808 so, that we can write",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; character set to memory"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; (No need to write SPACE it is clear char already)",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"8"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"48"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A

",
                        ),
                        (TokenKind::Comment, r"; Let's copy character set"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" HL"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"CHARS
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" B"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" CHARS_END-CHARS
COPYCHARS",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r",("),
                        (TokenKind::Plain, r"HL"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"DATAP"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" HL
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::Comment,
                            r"; Let's wait 8 clock cycles just in case VDP is not quick enough.",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DJNZ"),
                        (
                            TokenKind::Plain,
                            r" COPYCHARS

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Let's set write address to start of name table",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"XOR"),
                        (
                            TokenKind::Plain,
                            r" A
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"40"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"C"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A

",
                        ),
                        (TokenKind::Comment, r"; Let's put characters to screen"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" HL"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"ORDER
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" B"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"ORDER_END-ORDER
COPYORDER",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r",("),
                        (TokenKind::Plain, r"HL"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"DATAP"),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"A
",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" HL

",
                        ),
                        (TokenKind::Keyword, r"JR"),
                        (
                            TokenKind::Plain,
                            r" OVERNMI
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"NOP"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Here is address #66, that is entry for NMI",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"RETN"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Comment, r";Return from NMI"),
                        (
                            TokenKind::Plain,
                            r"

OVERNMI",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DJNZ"),
                        (
                            TokenKind::Plain,
                            r" COPYORDER

",
                        ),
                        (TokenKind::Comment, r"; The end"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"HALT"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Comment, r"; Character set:"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; --------------"),
                        (
                            TokenKind::Plain,
                            r"
ORDER",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"3"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"3"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"5"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"3"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"7"),
                        (
                            TokenKind::Plain,
                            r"
ORDER_END",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"

CHARS",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Comment, r"; H"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"11111000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; e"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"11111000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; l"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; o"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; W"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10101000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10101000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"11011000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Comment, r"; r"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10110000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"11001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; d"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01101000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10011000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10001000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"10011000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01101000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
chars_end",
                        ),
                        (TokenKind::Punctuation, r":"),
                    ],
                )
            },
        }],
    };

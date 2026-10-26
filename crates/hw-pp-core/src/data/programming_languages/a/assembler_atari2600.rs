//! Hello-world snippet for `ProgrammingLanguageName::AssemblerAtari2600`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Atari 2600.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Atari%202600.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerAtari2600 => "Assembler Atari 2600",
    versions: [AssemblerAtari2600 => "Assembler Atari 2600"],
    extensions: [],
}

const ASSEMBLER_ATARI2600: &str = concat!(
    r#"
; A ""#,
    output_for_hello_world!(),
    r#"" which illustrates an Atari 2600 programming
; introduction talk (slides at http://slideshare.net/chesterbr).
;
; This is free software (see license below). Build it with DASM
; (http://dasm-dillon.sourceforge.net/), by running:
;
;   dasm hello.asm -ohello.bin -f3
;

    PROCESSOR 6502
    INCLUDE "vcs.h"

    ORG $F000
StartFrame:
    lda #%00000010  
    sta VSYNC
    REPEAT 3      
        sta WSYNC   
    REPEND
    lda #0
    sta VSYNC       

PreparePlayfield:  
    lda #$00       
    sta ENABL      
    sta ENAM0
    sta ENAM1
    sta GRP0
    sta GRP1
    sta COLUBK      
    sta PF0         
    sta PF2
    lda #$FF       
    sta COLUPF
    lda #$00        
    sta CTRLPF
    ldx #0          
    REPEAT 37       
        sta WSYNC   
    REPEND
    lda #0          
    sta VBLANK

Scanline:
    cpx #174        
    bcs ScanlineEnd 
    txa             
    lsr             
    tay             
    lda Phrase,y   
    sta PF1       
ScanlineEnd:
    sta WSYNC    
    inx             
    cpx #191
    bne Scanline

Overscan:
    lda #%01000010  
    sta VBLANK      
    REPEAT 30       
        sta WSYNC
    REPEND
    jmp StartFrame 

Phrase:
    .BYTE %00000000 
    .BYTE %01000010
    .BYTE %01111110
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01111110
    .BYTE %01000000
    .BYTE %01111100
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01111110
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01111110
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01111110
    .BYTE %00000000 
    .BYTE %00000000
    .BYTE %00111100
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %00111100
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01011010
    .BYTE %00100100
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %00111100
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %00111100
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01111100
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01111100
    .BYTE %01000100
    .BYTE %01000010
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01000000
    .BYTE %01111110
    .BYTE %00000000
    .BYTE %00000000 
    .BYTE %01111000
    .BYTE %01000100
    .BYTE %01000010
    .BYTE %01000010
    .BYTE %01000100
    .BYTE %01111000
    .BYTE %00000000)

    ORG $FFFA
    .WORD StartFrame     
    .WORD StartFrame     
    .WORD StartFrame   

    END

;
; Copyright 2011-2013 Carlos Duarte do Nascimento (Chester). All rights reserved.
;
; Redistribution and use in source and binary forms, with or without modification, are
; permitted provided that the following conditions are met:
;
;    1. Redistributions of source code must retain the above copyright notice, this list of
;       conditions and the following disclaimer.
;
;    2. Redistributions in binary form must reproduce the above copyright notice, this list
;       of conditions and the following disclaimer in the documentation and/or other materials
;       provided with the distribution.
;
; THIS SOFTWARE IS PROVIDED BY CHESTER ''AS IS'' AND ANY EXPRESS OR IMPLIED
; WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
; FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> OR
; CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
; CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
; SERVICES;  LOSS OF USE, DATA, OR PROFITS;  OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
; ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
; NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
; ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
;
; The views and conclusions contained in the software and documentation are those of the
; authors and should not be interpreted as representing official policies, either expressed
; or implied, of Chester.
;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerAtari2600,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerAtari2600,
            syntax: ASSEMBLER_ATARI2600,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ATARI2600,
                    [
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"; A ""#,
                                output_for_hello_world!(),
                                r#"" which illustrates an Atari 2600 programming"#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; introduction talk (slides at http://slideshare.net/chesterbr).",
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
                            r"; This is free software (see license below). Build it with DASM",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; (http://dasm-dillon.sourceforge.net/), by running:",
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
                        (TokenKind::Comment, r";   dasm hello.asm -ohello.bin -f3"),
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
                        (TokenKind::Keyword, r"PROCESSOR"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"6502"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"INCLUDE"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""vcs.h""#),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"ORG"),
                        (
                            TokenKind::Plain,
                            r" $F000
StartFrame",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #%"),
                        (TokenKind::Number, r"00000010"),
                        (
                            TokenKind::Plain,
                            r"  
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" VSYNC
    ",
                        ),
                        (TokenKind::Keyword, r"REPEAT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"3"),
                        (
                            TokenKind::Plain,
                            r"      
        ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" WSYNC   
    ",
                        ),
                        (TokenKind::Keyword, r"REPEND"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" VSYNC       

PreparePlayfield",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"  
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #$"),
                        (TokenKind::Number, r"00"),
                        (
                            TokenKind::Plain,
                            r"       
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" ENABL      
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" ENAM0
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" ENAM1
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" GRP0
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" GRP1
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" COLUBK      
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" PF0         
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" PF2
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (
                            TokenKind::Plain,
                            r" #$FF       
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" COLUPF
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #$"),
                        (TokenKind::Number, r"00"),
                        (
                            TokenKind::Plain,
                            r"        
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" CTRLPF
    ",
                        ),
                        (TokenKind::Keyword, r"ldx"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"          
    ",
                        ),
                        (TokenKind::Keyword, r"REPEAT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"37"),
                        (
                            TokenKind::Plain,
                            r"       
        ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" WSYNC   
    ",
                        ),
                        (TokenKind::Keyword, r"REPEND"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"          
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" VBLANK

Scanline",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"cpx"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"174"),
                        (
                            TokenKind::Plain,
                            r"        
    ",
                        ),
                        (TokenKind::Keyword, r"bcs"),
                        (
                            TokenKind::Plain,
                            r" ScanlineEnd 
    ",
                        ),
                        (TokenKind::Keyword, r"txa"),
                        (
                            TokenKind::Plain,
                            r"             
    ",
                        ),
                        (TokenKind::Keyword, r"lsr"),
                        (
                            TokenKind::Plain,
                            r"             
    ",
                        ),
                        (TokenKind::Keyword, r"tay"),
                        (
                            TokenKind::Plain,
                            r"             
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" Phrase"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"y   
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" PF1       
ScanlineEnd",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" WSYNC    
    ",
                        ),
                        (TokenKind::Keyword, r"inx"),
                        (
                            TokenKind::Plain,
                            r"             
    ",
                        ),
                        (TokenKind::Keyword, r"cpx"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"191"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"bne"),
                        (
                            TokenKind::Plain,
                            r" Scanline

Overscan",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r" #%"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"  
    ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" VBLANK      
    ",
                        ),
                        (TokenKind::Keyword, r"REPEAT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"30"),
                        (
                            TokenKind::Plain,
                            r"       
        ",
                        ),
                        (TokenKind::Keyword, r"sta"),
                        (
                            TokenKind::Plain,
                            r" WSYNC
    ",
                        ),
                        (TokenKind::Keyword, r"REPEND"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r" StartFrame 

Phrase",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01011010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00100100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111110"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000010"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01000100"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"01111000"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Macro, r".BYTE"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Number, r"00000000"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"ORG"),
                        (
                            TokenKind::Plain,
                            r" $FFFA
    ",
                        ),
                        (TokenKind::Macro, r".WORD"),
                        (
                            TokenKind::Plain,
                            r" StartFrame     
    ",
                        ),
                        (TokenKind::Macro, r".WORD"),
                        (
                            TokenKind::Plain,
                            r" StartFrame     
    ",
                        ),
                        (TokenKind::Macro, r".WORD"),
                        (
                            TokenKind::Plain,
                            r" StartFrame   

    ",
                        ),
                        (TokenKind::Keyword, r"END"),
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
                            r"; Copyright 2011-2013 Carlos Duarte do Nascimento (Chester). All rights reserved.",
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
                            r"; Redistribution and use in source and binary forms, with or without modification, are",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; permitted provided that the following conditions are met:",
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
                            r";    1. Redistributions of source code must retain the above copyright notice, this list of",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";       conditions and the following disclaimer.",
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
                            r";    2. Redistributions in binary form must reproduce the above copyright notice, this list",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";       of conditions and the following disclaimer in the documentation and/or other materials",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r";       provided with the distribution.",
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
                            r"; THIS SOFTWARE IS PROVIDED BY CHESTER ''AS IS'' AND ANY EXPRESS OR IMPLIED",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> OR",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; SERVICES;  LOSS OF USE, DATA, OR PROFITS;  OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.",
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
                            r"; The views and conclusions contained in the software and documentation are those of the",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; authors and should not be interpreted as representing official policies, either expressed",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; or implied, of Chester."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::AssemblerDcpu16`.
//!
//! Imported from leachim6/hello-world (`a/Assembler DCPU16.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20DCPU16.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: AssemblerDcpu16 => "Assembler DCPU16",
    versions: [AssemblerDcpu16 => "Assembler DCPU16"],
    extensions: [],
}

const ASSEMBLER_DCPU16: &str = r";;;;;;;;;;;;;;;;;;;
jsr init_devices
jsr Hello_World
set PC,end
;;;;;;;;;;;;;;;;;;;
:dev_screen   dat 0
:device_count dat 0
;;;;;;;;;;;;;;;;;;;
:find_devices
hwn [device_count]
set I,0
:find_devices_loop
hwq i
ife A,0xf615
    set [dev_screen],1
add I,1
ifn I,device_count
    set PC,find_devices_loop
set PC,pop
:init_devices
jsr find_devices
;set up display 
set A,0
set B,0x8000
hwi [dev_screen]
set PC,pop

:Hello_World
set [0x8000],0x1048
set [0x8001],0x1065
set [0x8002],0x106c
set [0x8003],0x106c
set [0x8004],0x106f
set [0x8005],0x102c
set [0x8006],0
set [0x8007],0x1077
set [0x8008],0x106f
set [0x8009],0x1072
set [0x800a],0x106c
set [0x800b],0x1064
set [0x800c],0x1021
set PC,pop

:end
set PC,end";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerDcpu16,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerDcpu16,
            syntax: ASSEMBLER_DCPU16,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_DCPU16,
                    [
                        (TokenKind::Comment, r";;;;;;;;;;;;;;;;;;;"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r" init_devices
",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r" Hello_World
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"end
",
                        ),
                        (TokenKind::Comment, r";;;;;;;;;;;;;;;;;;;"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"dev_screen   dat "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"device_count dat "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";;;;;;;;;;;;;;;;;;;"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"find_devices
",
                        ),
                        (TokenKind::Keyword, r"hwn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"device_count"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" I"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"find_devices_loop
",
                        ),
                        (TokenKind::Keyword, r"hwq"),
                        (
                            TokenKind::Plain,
                            r" i
",
                        ),
                        (TokenKind::Keyword, r"ife"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0xf615"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"dev_screen"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"add"),
                        (TokenKind::Plain, r" I"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"ifn"),
                        (TokenKind::Plain, r" I"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"device_count
    ",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"find_devices_loop
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"pop
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"init_devices
",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r" find_devices
",
                        ),
                        (TokenKind::Comment, r";set up display "),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" B"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0x8000"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"hwi"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"dev_screen"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"pop

",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"Hello_World
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8000"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1048"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8001"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1065"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8002"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x106c"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8003"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x106c"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8004"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x106f"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8005"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x102c"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8006"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8007"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1077"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8008"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x106f"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x8009"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1072"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x800a"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x106c"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x800b"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1064"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Number, r"0x800c"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Number, r"0x1021"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"pop

",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"end
",
                        ),
                        (TokenKind::Keyword, r"set"),
                        (TokenKind::Plain, r" PC"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"end"),
                    ],
                )
            },
        }],
    };

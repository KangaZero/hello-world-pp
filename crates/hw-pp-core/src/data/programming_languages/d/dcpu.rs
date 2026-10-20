//! Hello-world snippet for `ProgrammingLanguageName::Dcpu`.
//!
//! Imported from leachim6/hello-world (`d/DCPU.dasm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DCPU.dasm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dcpu => "DCPU",
    versions: [Dcpu => "DCPU"],
    extensions: [Dasm => "dasm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dcpu,
        extensions: &[FileExtension::Dasm],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dcpu,
            syntax: concat!(
                r#"  set i, 0xf615
  set j, 0x7349
  hwn z
  sub z, 1

:device_detect_loop
  hwq z

  ife a, i
    ife b, j
      set pc, device_detect_ret

  sub z, 1
  ifa z, 0xffff
    set pc, device_detect_loop

:device_detect_ret
  set a, 0
  set b, [vidmem]
  hwi z

  set i, [vidmem]
  set j, string
  set c, 0xc

:memcpy_loop
  bor [j], 0xf000 ; OR with font style.
  sti [i], [j]
  sub c, 1
  ife c, 0
    sub pc, 1
  set pc, memcpy_loop

:display dat 0
:vidmem dat 0x8000
:string dat ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"  "),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" i"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0xf615"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" j"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x7349"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"hwn"),
                (
                    TokenKind::Plain,
                    r" z
  ",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" z"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"device_detect_loop
  ",
                ),
                (TokenKind::Keyword, r"hwq"),
                (
                    TokenKind::Plain,
                    r" z

  ",
                ),
                (TokenKind::Keyword, r"ife"),
                (TokenKind::Plain, r" a"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" i
    ",
                ),
                (TokenKind::Keyword, r"ife"),
                (TokenKind::Plain, r" b"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" j
      ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" pc"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" device_detect_ret

  ",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" z"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"ifa"),
                (TokenKind::Plain, r" z"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0xffff"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" pc"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" device_detect_loop

",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"device_detect_ret
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" a"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" b"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"vidmem"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"hwi"),
                (
                    TokenKind::Plain,
                    r" z

  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" i"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"vidmem"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" j"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"string"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" c"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0xc"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"memcpy_loop
  ",
                ),
                (TokenKind::Keyword, r"bor"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"j"),
                (TokenKind::Punctuation, r"],"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0xf000"),
                (TokenKind::Plain, r" "),
                (TokenKind::Comment, r"; OR with font style."),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"sti"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"i"),
                (TokenKind::Punctuation, r"],"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"j"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" c"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"ife"),
                (TokenKind::Plain, r" c"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" pc"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"set"),
                (TokenKind::Plain, r" pc"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" memcpy_loop

",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Function, r"display"),
                (TokenKind::Plain, r" dat "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"vidmem dat "),
                (TokenKind::Number, r"0x8000"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Keyword, r"string"),
                (TokenKind::Plain, r" dat "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };

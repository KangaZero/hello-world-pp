//! Hello-world snippet for `ProgrammingLanguageName::Il`.
//!
//! Imported from leachim6/hello-world (`i/Il.il` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Il.il>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Il => "Il",
    versions: [Il => "Il"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Il,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Il,
            syntax: concat!(
                r#"
// assembly references
.assembly extern mscorlib
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )
  .ver 4:0:0:0
}

// assembly manifest
.assembly ilhw
{
  .custom instance void [mscorlib]System.Runtime.Versioning.TargetFrameworkAttribute::.ctor(string)
    = ( 01 00 1C 2E 4E 45 54 46 72 61 6D 65 77 6F 72 6B
        2C 56 65 72 73 69 6F 6E 3D 76 34 2E 35 2E 33 01
        00 54 0E 14 46 72 61 6D 65 77 6F 72 6B 44 69 73
        70 6C 61 79 4E 61 6D 65 14 2E 4E 45 54 20 46 72
        61 6D 65 77 6F 72 6B 20 34 2E 35 2E 33 )


  .hash algorithm 0x00008004
  .ver 0:0:0:0
}

.module         ilhw.dll
.imagebase      0x00400000
.file alignment 0x00000200
.stackreserve   0x00100000
.subsystem      0x0003
.corflags       0x00000001


.class public abstract auto ansi sealed ILHW.Program
  extends [mscorlib]System.Object // nope, not implicit P:
{

  .method public hidebysig static void Main(class [mscorlib]System.String[] args) cil managed
  {
    .entrypoint // this method is the entry point of the program
    .maxstack 1 // the maximum amount of objects on the stack is 1.

    ldstr ""#,
                output_for_hello_world!(),
                r#""
    call void [mscorlib]System.Console::WriteLine(string)

    ret
  }

}"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Comment, r"// assembly references"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"assembly "),
                (TokenKind::Keyword, r"extern"),
                (
                    TokenKind::Plain,
                    r" mscorlib
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"publickeytoken = "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"B7 7A 5C "),
                (TokenKind::Number, r"56"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"19"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"34"),
                (TokenKind::Plain, r" E0 "),
                (TokenKind::Number, r"89"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"ver "),
                (TokenKind::Number, r"4"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Comment, r"// assembly manifest"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"assembly ilhw
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"custom instance "),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"mscorlib"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Runtime"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Versioning"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"TargetFrameworkAttribute"),
                (TokenKind::Punctuation, r"::."),
                (TokenKind::Function, r"ctor"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"string"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    = ",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"01"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"00"),
                (TokenKind::Plain, r" 1C 2E 4E "),
                (TokenKind::Number, r"45"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"54"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"46"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"72"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"61"),
                (TokenKind::Plain, r" 6D "),
                (TokenKind::Number, r"65"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"77"),
                (TokenKind::Plain, r" 6F "),
                (TokenKind::Number, r"72"),
                (
                    TokenKind::Plain,
                    r" 6B
        2C ",
                ),
                (TokenKind::Number, r"56"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"65"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"72"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"73"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"69"),
                (TokenKind::Plain, r" 6F 6E 3D "),
                (TokenKind::Number, r"76"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"34"),
                (TokenKind::Plain, r" 2E "),
                (TokenKind::Number, r"35"),
                (TokenKind::Plain, r" 2E "),
                (TokenKind::Number, r"33"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"01"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Number, r"00"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"54"),
                (TokenKind::Plain, r" 0E "),
                (TokenKind::Number, r"14"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"46"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"72"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"61"),
                (TokenKind::Plain, r" 6D "),
                (TokenKind::Number, r"65"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"77"),
                (TokenKind::Plain, r" 6F "),
                (TokenKind::Number, r"72"),
                (TokenKind::Plain, r" 6B "),
                (TokenKind::Number, r"44"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"69"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"73"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Number, r"70"),
                (TokenKind::Plain, r" 6C "),
                (TokenKind::Number, r"61"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"79"),
                (TokenKind::Plain, r" 4E "),
                (TokenKind::Number, r"61"),
                (TokenKind::Plain, r" 6D "),
                (TokenKind::Number, r"65"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"14"),
                (TokenKind::Plain, r" 2E 4E "),
                (TokenKind::Number, r"45"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"54"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"20"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"46"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"72"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Number, r"61"),
                (TokenKind::Plain, r" 6D "),
                (TokenKind::Number, r"65"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"77"),
                (TokenKind::Plain, r" 6F "),
                (TokenKind::Number, r"72"),
                (TokenKind::Plain, r" 6B "),
                (TokenKind::Number, r"20"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"34"),
                (TokenKind::Plain, r" 2E "),
                (TokenKind::Number, r"35"),
                (TokenKind::Plain, r" 2E "),
                (TokenKind::Number, r"33"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"


  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"hash algorithm "),
                (TokenKind::Number, r"0x00008004"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"ver "),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Keyword, r"module"),
                (TokenKind::Plain, r"         ilhw"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"dll
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"imagebase      "),
                (TokenKind::Number, r"0x00400000"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"file alignment "),
                (TokenKind::Number, r"0x00000200"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"stackreserve   "),
                (TokenKind::Number, r"0x00100000"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"subsystem      "),
                (TokenKind::Number, r"0x0003"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"corflags       "),
                (TokenKind::Number, r"0x00000001"),
                (
                    TokenKind::Plain,
                    r"


",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"abstract"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"auto"),
                (TokenKind::Plain, r" ansi "),
                (TokenKind::Keyword, r"sealed"),
                (TokenKind::Plain, r" ILHW"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"Program
  ",
                ),
                (TokenKind::Keyword, r"extends"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"mscorlib"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Object "),
                (TokenKind::Comment, r"// nope, not implicit P:"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"

  ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"method "),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" hidebysig "),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Main"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"mscorlib"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"String"),
                (TokenKind::Punctuation, r"[]"),
                (TokenKind::Plain, r" args"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r" cil managed
  ",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"entrypoint "),
                (
                    TokenKind::Comment,
                    r"// this method is the entry point of the program",
                ),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"maxstack "),
                (TokenKind::Number, r"1"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::Comment,
                    r"// the maximum amount of objects on the stack is 1.",
                ),
                (
                    TokenKind::Plain,
                    r"

    ldstr ",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
    call ",
                ),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"mscorlib"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Console"),
                (TokenKind::Punctuation, r"::"),
                (TokenKind::Function, r"WriteLine"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"string"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

    ret
  ",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };

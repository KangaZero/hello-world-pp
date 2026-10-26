//! Hello-world snippet for `ProgrammingLanguageName::VaxMacro`.
//!
//! Imported from the Hello World Collection (`files/VAX&nbsp;Macro.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VAX%26nbsp%3BMacro.txt>
//! - <http://helloworldcollection.de/#VAX%C2%A0Macro>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VaxMacro => "VAX Macro",
    versions: [VaxMacro => "VAX Macro"],
    extensions: [],
}

const VAX_MACRO: &str = concat!(
    output_for_hello_world!(),
    r" in VAX Macro.

        .title  helloworld
        .ident  /",
    output_for_hello_world!(),
    r"/
;
        .library        /sys$library:lib/
        $libdef
        $lib$routinesdef


        .psect  $data,wrt,noshr,noexe,long

hello:  .ascid  /",
    output_for_hello_world!(),
    r"/

        .psect  $code,nowrt,shr,exe,long

        .entry  helloworld,^m<r9,r10,r11>

        pushaq  hello                   ; output the
message
        calls   #1,g^lib$put_output     ;

        ret                             ; GTFOH
        .end    helloworld              ;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VaxMacro,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VaxMacro,
            syntax: VAX_MACRO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VAX_MACRO,
                    [
                        (
                            TokenKind::Keyword,
                            concat!(output_for_hello_world!(), r" in VAX Macro"),
                        ),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Macro, r".title"),
                        (
                            TokenKind::Plain,
                            r"  helloworld
        ",
                        ),
                        (TokenKind::Macro, r".ident"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"  /",
                                output_for_hello_world!(),
                                r"/
"
                            ),
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Macro, r".library"),
                        (TokenKind::Plain, r"        /sys$library"),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"lib/
        $libdef
        $lib$routinesdef


        ",
                        ),
                        (TokenKind::Macro, r".psect"),
                        (TokenKind::Plain, r"  $data"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"wrt"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"noshr"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"noexe"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"long

hello",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"  "),
                        (TokenKind::Macro, r".ascid"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"  /",
                                output_for_hello_world!(),
                                r"/

        "
                            ),
                        ),
                        (TokenKind::Macro, r".psect"),
                        (TokenKind::Plain, r"  $code"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"nowrt"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"shr"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"exe"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"long

        ",
                        ),
                        (TokenKind::Macro, r".entry"),
                        (TokenKind::Plain, r"  helloworld"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"^m<r9"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"r10"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"r11>

        ",
                        ),
                        (TokenKind::Keyword, r"pushaq"),
                        (TokenKind::Plain, r"  hello                   "),
                        (TokenKind::Comment, r"; output the"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"message"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"calls"),
                        (TokenKind::Plain, r"   #"),
                        (TokenKind::Number, r"1"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"g^lib$put_output     "),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"ret"),
                        (TokenKind::Plain, r"                             "),
                        (TokenKind::Comment, r"; GTFOH"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Macro, r".end"),
                        (TokenKind::Plain, r"    helloworld              "),
                        (TokenKind::Comment, r";"),
                    ],
                )
            },
        }],
    };

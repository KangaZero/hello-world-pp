//! Hello-world snippet for `ProgrammingLanguageName::Vax11Macro`.
//!
//! Imported from the Hello World Collection (`files/VAX-11&nbsp;Macro.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VAX-11%26nbsp%3BMacro.txt>
//! - <http://helloworldcollection.de/#VAX-11%C2%A0Macro>
//! - <https://en.wikipedia.org/wiki/VAX_MACRO> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vax11Macro => "VAX-11 Macro",
    versions: [Vax11Macro => "VAX-11 Macro"],
    extensions: [],
}

const VAX11_MACRO: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in VAX-11 MACRO

        .title hello
term_name:      .ascid /SYS$INPUT/
term_chan:      .blkw 1
out_iosb:       .blkq 1
msg:    .asciz  /",
    output_for_hello_world!(),
    r"/

        .entry start,0

        ; establish a channel for terminal I/O
        $assign_s devnam=term_name,-
                chan=term_chan
        blbc r0,error

        ; queue the I/O request
        $qio_s chan=term_chan,-
                func=#io$_writevblk,-
                iosb=out_iosb,-
                p1=msg,-
                p2=#13
        blbc r0,error

        $exit_s ; normal exit

error:  halt ; error condition

        .end start"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vax11Macro,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vax11Macro,
            syntax: VAX11_MACRO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VAX11_MACRO,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"; ", output_for_hello_world!(), r" in VAX-11 MACRO"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Macro, r".title"),
                        (
                            TokenKind::Plain,
                            r" hello
term_name",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Macro, r".ascid"),
                        (
                            TokenKind::Plain,
                            r" /SYS$INPUT/
term_chan",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Macro, r".blkw"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
out_iosb",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"       "),
                        (TokenKind::Macro, r".blkq"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
msg",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"    "),
                        (TokenKind::Macro, r".asciz"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"  /",
                                output_for_hello_world!(),
                                r"/

        "
                            ),
                        ),
                        (TokenKind::Macro, r".entry"),
                        (TokenKind::Plain, r" start"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (
                            TokenKind::Comment,
                            r"; establish a channel for terminal I/O",
                        ),
                        (
                            TokenKind::Plain,
                            r"
        $assign_s devnam=term_name",
                        ),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"-
                ",
                        ),
                        (TokenKind::Keyword, r"chan"),
                        (
                            TokenKind::Plain,
                            r"=term_chan
        ",
                        ),
                        (TokenKind::Keyword, r"blbc"),
                        (TokenKind::Plain, r" r0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"error

        ",
                        ),
                        (TokenKind::Comment, r"; queue the I/O request"),
                        (
                            TokenKind::Plain,
                            r"
        $qio_s chan=term_chan",
                        ),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"-
                ",
                        ),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r"=#io$_writevblk"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"-
                ",
                        ),
                        (TokenKind::Keyword, r"iosb"),
                        (TokenKind::Plain, r"=out_iosb"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"-
                ",
                        ),
                        (TokenKind::Keyword, r"p1"),
                        (TokenKind::Plain, r"=msg"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"-
                ",
                        ),
                        (TokenKind::Keyword, r"p2"),
                        (TokenKind::Plain, r"=#"),
                        (TokenKind::Number, r"13"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"blbc"),
                        (TokenKind::Plain, r" r0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"error

        $exit_s ",
                        ),
                        (TokenKind::Comment, r"; normal exit"),
                        (
                            TokenKind::Plain,
                            r"

error",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"  halt "),
                        (TokenKind::Comment, r"; error condition"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Macro, r".end"),
                        (TokenKind::Plain, r" start"),
                    ],
                )
            },
        }],
    };

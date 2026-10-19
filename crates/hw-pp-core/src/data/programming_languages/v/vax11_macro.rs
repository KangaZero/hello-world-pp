//! Hello-world snippet for `ProgrammingLanguageName::Vax11Macro`.
//!
//! Imported from the Hello World Collection (`files/VAX-11&nbsp;Macro.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VAX-11%26nbsp%3BMacro.txt>
//! - <http://helloworldcollection.de/#VAX-11%C2%A0Macro>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vax11Macro => "VAX-11 Macro",
    versions: [Vax11Macro => "VAX-11 Macro"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vax11Macro,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vax11Macro,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };

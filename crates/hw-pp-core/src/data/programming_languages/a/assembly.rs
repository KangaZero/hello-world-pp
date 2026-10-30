//! Hello-world snippet for `ProgrammingLanguageName::Assembly`.
//!
//! Sources:
//! - <https://www.nasm.us/doc/>
//! - <https://man7.org/linux/man-pages/man2/write.2.html>
//! - <https://man7.org/linux/man-pages/man2/exit.2.html>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Assembly, Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Assembly => "Assembly",
    versions: [X8664LinuxSyscalls => "x86-64 (Linux syscalls)"],
    extensions: [Asm => "asm", S => "s"],
}

const X8664_LINUX_SYSCALLS: &str = concat!(
    r#"section .data
    msg db ""#,
    output_for_hello_world!(),
    r#"", 0x0a
    len equ $ - msg

section .text
    global _start

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, len
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembly,
        extensions: &[FileExtension::Asm, FileExtension::S],
        categories: &[Assembly, Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::X8664LinuxSyscalls,
            syntax: X8664_LINUX_SYSCALLS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    X8664_LINUX_SYSCALLS,
                    [
                        (TokenKind::Plain, "section .data\n    msg db "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Plain, ", "),
                        (TokenKind::Number, "0x0a"),
                        (TokenKind::Plain, "\n    len equ "),
                        (TokenKind::Keyword, "$"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "-"),
                        (
                            TokenKind::Plain,
                            " msg\n\nsection .text\n    global _start\n\n_start",
                        ),
                        (TokenKind::Keyword, ":"),
                        (TokenKind::Plain, "\n    mov rax, "),
                        (TokenKind::Number, "1"),
                        (TokenKind::Plain, "\n    mov rdi, "),
                        (TokenKind::Number, "1"),
                        (
                            TokenKind::Plain,
                            "\n    mov rsi, msg\n    mov rdx, len\n    syscall\n    mov rax, ",
                        ),
                        (TokenKind::Number, "60"),
                        (TokenKind::Plain, "\n    xor rdi, rdi\n    syscall"),
                    ],
                )
            },
        }],
    };

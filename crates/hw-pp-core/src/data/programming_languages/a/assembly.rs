//! Hello-world snippets for `ProgrammingLanguageName::Assembly`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembly => "Assembly",
    versions: [X8664LinuxSyscalls => "x86-64 (Linux syscalls)"],
    extensions: [Asm => "asm", S => "s"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembly,
        extensions: &[FileExtension::Asm, FileExtension::S],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::X8664LinuxSyscalls,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[
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
        }],
    };

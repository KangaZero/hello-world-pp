//! Hello-world snippet for `ProgrammingLanguageName::Webassembly`.
//!
//! Imported from leachim6/hello-world (`w/WebAssembly.wat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Webassembly => "WebAssembly",
    versions: [Webassembly => "WebAssembly"],
    extensions: [Wat => "wat"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Webassembly,
        extensions: &[FileExtension::Wat],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Webassembly,
            syntax: concat!(
                r#"(module
    (import "wasi_unstable" "fd_write"
        (func $fd_write (param i32 i32 i32 i32) (result i32))
    )

    (memory 1)
    (export "memory" (memory 0))

    (data (i32.const 0) "\08\00\00\00\0c\00\00\00"#,
                output_for_hello_world!(),
                r#"\n")

    (func $main (export "_start")
        i32.const 1
        i32.const 0
        i32.const 1
        i32.const 20
        call $fd_write
        drop
    )
)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(module
    (import "wasi_unstable" "fd_write"
        (func $fd_write (param i32 i32 i32 i32) (result i32))
    )

    (memory 1)
    (export "memory" (memory 0))

    (data (i32.const 0) "\08\00\00\00\0c\00\00\00"#,
                    output_for_hello_world!(),
                    r#"\n")

    (func $main (export "_start")
        i32.const 1
        i32.const 0
        i32.const 1
        i32.const 20
        call $fd_write
        drop
    )
)"#
                ),
            )],
        }],
    };

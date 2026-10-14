use crate::data::{common::DEFAULT_PROGRAMMING_LANGUAGE, programming_languages_token::TokenKind};

use super::lookup::LookupError;
use super::macros::{name_enum, output_for_hello_world};
use ProgrammingLanguageCategory::{Compiled, Esolang, Historical, Markup, Scripting};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgrammingLanguageCategory {
    Esolang,
    Scripting,
    Compiled,
    Markup,
    Historical,
}
/// Language/version resolution errors for the programming-language catalog.
pub type ProgrammingLookupError = LookupError<ProgrammingLanguageName, ProgrammingLanguageDialect>;

/// Errors that can occur while looking up hello-world source snippets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ProgrammingLanguageError {
    #[error(transparent)]
    Lookup(#[from] ProgrammingLookupError),

    /// No recorded language uses the extension.
    #[error("no recorded language has extension {0}")]
    UnknownExtension(FileExtension),

    /// The language is recorded but lists no file extensions.
    #[error("{0} has no recorded file extensions")]
    NoExtensions(ProgrammingLanguageName),

    /// The extension was given alongside a language that does not use it.
    #[error("{extension} is not an extension of {language}")]
    ExtensionMismatch {
        language: ProgrammingLanguageName,
        extension: FileExtension,
    },

    /// Should never happen but just in case a langauge has no extension(s)
    #[error("no recorded extension for language: {0}")]
    MissingExtension(ProgrammingLanguageName),
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum ProgrammingLanguageName: ParseProgrammingLanguageNameError("programming language") {
        #[default]
        C => "C",
        Cpp => "C++",
        CSharp => "C#",
        Java => "Java",
        Python => "Python",
        JavaScript => "JavaScript",
        TypeScript => "TypeScript",
        Rust => "Rust",
        Go => "Go",
        Ruby => "Ruby",
        Php => "PHP",
        Swift => "Swift",
        Kotlin => "Kotlin",
        Scala => "Scala",
        Perl => "Perl",
        Lua => "Lua",
        R => "R",
        Dart => "Dart",
        Elixir => "Elixir",
        Erlang => "Erlang",
        Haskell => "Haskell",
        Clojure => "Clojure",
        FSharp => "F#",
        OCaml => "OCaml",
        Zig => "Zig",
        Nim => "Nim",
        Julia => "Julia",
        Bash => "Bash",
        PowerShell => "PowerShell",
        Assembly => "Assembly",
        Fortran => "Fortran",
        Cobol => "COBOL",
        Ada => "Ada",
        Pascal => "Pascal",
        Basic => "BASIC",
        VisualBasic => "Visual Basic",
        Groovy => "Groovy",
        ObjectiveC => "Objective-C",
        D => "D",
        Crystal => "Crystal",
        Elm => "Elm",
        Racket => "Racket",
        CommonLisp => "Common Lisp",
        Scheme => "Scheme",
        Prolog => "Prolog",
        Sql => "SQL",
        Matlab => "MATLAB",
        Solidity => "Solidity",
        Html => "HTML",
        Css => "CSS",
        Xml => "XML",
        Markdown => "Markdown",
        Latex => "LaTeX",
        Brainfuck => "Brainfuck",
        Lolcode => "LOLCODE",
        Befunge => "Befunge",
        Apl => "APL",
        Algol => "ALGOL",
    }
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ProgrammingLanguageDialect: ParseLanguageVersionDialectError("language version") {
        C99 => "C99",
        Cpp98 => "C++98",
        Cpp20 => "C++20",
        CSharp10 => "C# 10",
        Java17 => "Java 17",
        Java25CompactSourceFile => "Java 25 (compact source file)",
        Python2 => "Python 2",
        Python3 => "Python 3",
        JavaScriptEs2020 => "ES2020",
        TypeScript5 => "TypeScript 5",
        Rust2021 => "Rust 2021",
        Go1 => "Go 1",
        Ruby3 => "Ruby 3",
        Php8 => "PHP 8",
        Swift5 => "Swift 5",
        Kotlin19 => "Kotlin 1.9",
        Scala3 => "Scala 3",
        Perl5 => "Perl 5",
        Lua54 => "Lua 5.4",
        R4 => "R 4",
        Dart3 => "Dart 3",
        Elixir1 => "Elixir 1",
        ErlangOtp => "Erlang/OTP",
        Haskell2010 => "Haskell 2010",
        Clojure111 => "Clojure 1.11",
        FSharp8 => "F# 8",
        OCaml5 => "OCaml 5",
        Zig013 => "Zig 0.13",
        Nim2 => "Nim 2",
        Julia110 => "Julia 1.10",
        PosixShell => "POSIX shell",
        PowerShell7 => "PowerShell 7",
        X8664LinuxSyscalls => "x86-64 (Linux syscalls)",
        Fortran90 => "Fortran 90",
        Cobol85 => "COBOL 85",
        Ada2012 => "Ada 2012",
        Pascal => "Pascal",
        ClassicBasic => "Classic BASIC",
        VbNet => "VB.NET",
        Groovy4 => "Groovy 4",
        ObjectiveC2 => "Objective-C 2.0",
        D2 => "D 2",
        Crystal1 => "Crystal 1",
        Elm019 => "Elm 0.19",
        Racket8 => "Racket 8",
        AnsiCommonLisp => "ANSI Common Lisp",
        R7rs => "R7RS",
        IsoProlog => "ISO Prolog",
        Sql2016 => "SQL:2016",
        Matlab => "MATLAB",
        Solidity08 => "Solidity 0.8",
        Html5 => "HTML5",
        Css3 => "CSS3",
        Xml10 => "XML 1.0",
        CommonMark => "CommonMark",
        Latex2e => "LaTeX2e",
        Brainfuck => "Brainfuck",
        Lolcode12 => "LOLCODE 1.2",
        Befunge93 => "Befunge-93",
        Apl => "APL",
        Algol60 => "ALGOL 60",
    }
}

name_enum! {
    /// A source file extension, without the leading dot. Several languages may
    /// share one (e.g. `m` is both Objective-C and MATLAB).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FileExtension: ParseFileExtensionError("file extension") {
        C => "c",
        H => "h",
        Cpp => "cpp",
        Cc => "cc",
        Cxx => "cxx",
        Hpp => "hpp",
        Cs => "cs",
        Java => "java",
        Py => "py",
        Js => "js",
        Mjs => "mjs",
        Cjs => "cjs",
        Ts => "ts",
        Rs => "rs",
        Go => "go",
        Rb => "rb",
        Php => "php",
        Swift => "swift",
        Kt => "kt",
        Kts => "kts",
        Scala => "scala",
        Sc => "sc",
        Pl => "pl",
        Pm => "pm",
        Lua => "lua",
        R => "r",
        Dart => "dart",
        Ex => "ex",
        Exs => "exs",
        Erl => "erl",
        Hrl => "hrl",
        Hs => "hs",
        Lhs => "lhs",
        Clj => "clj",
        Cljs => "cljs",
        Cljc => "cljc",
        Fs => "fs",
        Fsx => "fsx",
        Ml => "ml",
        Mli => "mli",
        Zig => "zig",
        Nim => "nim",
        Jl => "jl",
        Sh => "sh",
        Bash => "bash",
        Ps1 => "ps1",
        Psm1 => "psm1",
        Asm => "asm",
        S => "s",
        F => "f",
        F90 => "f90",
        F95 => "f95",
        Cob => "cob",
        Cbl => "cbl",
        Adb => "adb",
        Ads => "ads",
        Pas => "pas",
        Bas => "bas",
        Vb => "vb",
        Groovy => "groovy",
        Gvy => "gvy",
        M => "m",
        Mm => "mm",
        D => "d",
        Cr => "cr",
        Elm => "elm",
        Rkt => "rkt",
        Lisp => "lisp",
        Lsp => "lsp",
        Cl => "cl",
        Scm => "scm",
        Ss => "ss",
        Pro => "pro",
        Sql => "sql",
        Sol => "sol",
        Html => "html",
        Htm => "htm",
        Css => "css",
        Xml => "xml",
        Md => "md",
        Markdown => "markdown",
        Tex => "tex",
        Bf => "bf",
        Lol => "lol",
        Apl => "apl",
        Alg => "alg",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguageVersion {
    pub label: ProgrammingLanguageDialect,
    pub syntax: &'static str,
    pub tokenized_syntax: &'static [(TokenKind, &'static str)],
}

#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguage {
    pub name: ProgrammingLanguageName,
    pub extensions: &'static [FileExtension],
    pub categories: &'static [ProgrammingLanguageCategory],
    pub versions: &'static [ProgrammingLanguageVersion],
}

pub static PROGRAMMING_LANGUAGES: &[ProgrammingLanguage] = &[
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C,
        extensions: &[FileExtension::C, FileExtension::H],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::C99,
            syntax: concat!(
                r#"#include <stdio.h>

int main(void) {
    printf(""#,
                output_for_hello_world!(),
                r#"\n");
    return 0;
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#include"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "<stdio.h>"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "int"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Type, "void"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "printf"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "return"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "0"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cpp,
        extensions: &[
            FileExtension::Cpp,
            FileExtension::Cc,
            FileExtension::Cxx,
            FileExtension::Hpp,
        ],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Cpp98,
                syntax: concat!(
                    r#"#include <iostream>

int main() {
    std::cout << ""#,
                    output_for_hello_world!(),
                    r#"" << std::endl;
    return 0;
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "#include"),
                    (TokenKind::Plain, " "),
                    (TokenKind::String, "<iostream>"),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Type, "int"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "cout "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "endl"),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "return"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Number, "0"),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Cpp20,
                syntax: concat!(
                    r#"#include <iostream>

int main() {
    std::cout << ""#,
                    output_for_hello_world!(),
                    r#"\n";
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "#include"),
                    (TokenKind::Plain, " "),
                    (TokenKind::String, "<iostream>"),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Type, "int"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "cout "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), r#"\n""#),
                    ),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSharp,
        extensions: &[FileExtension::Cs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CSharp10,
            syntax: concat!(
                r#"Console.WriteLine(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "Console"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "WriteLine"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &[FileExtension::Java],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java17,
                syntax: concat!(
                    r#"class Main {
    public static void main(String[] args) {
        System.out.println(""#,
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "class"),
                    (TokenKind::Plain, " Main "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "public"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "static"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "("),
                    (TokenKind::Plain, "String"),
                    (TokenKind::Keyword, "[]"),
                    (TokenKind::Plain, " args"),
                    (TokenKind::Punctuation, ")"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n        System"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Plain, "out"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Punctuation, "}"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java25CompactSourceFile,
                syntax: concat!(
                    r#"void main() {
    IO.println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    IO"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Python,
        extensions: &[FileExtension::Py],
        categories: &[Scripting],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python2,
                syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "print"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python3,
                syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                tokenized_syntax: &[
                    (TokenKind::Function, "print"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ")"),
                ],
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaScript,
        extensions: &[FileExtension::Js, FileExtension::Mjs, FileExtension::Cjs],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JavaScriptEs2020,
            syntax: concat!(r#"console.log(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Type, "console"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "log"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TypeScript,
        extensions: &[FileExtension::Ts],
        categories: &[Scripting, Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TypeScript5,
            syntax: concat!(
                r#"const greeting: string = ""#,
                output_for_hello_world!(),
                r#"";
console.log(greeting);"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "const"),
                (TokenKind::Plain, " greeting"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "string"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "="),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\nconsole"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "log"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rust,
        extensions: &[FileExtension::Rs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rust2021,
            syntax: concat!(
                r#"fn main() {
    println!(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "fn"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Macro, "println!"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Go,
        extensions: &[FileExtension::Go],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Go1,
            syntax: concat!(
                r#"package main

import "fmt"

func main() {
    fmt.Println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "package"),
                (TokenKind::Plain, " main\n\n"),
                (TokenKind::Keyword, "import"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "\"fmt\""),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "func"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    fmt"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "Println"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ruby,
        extensions: &[FileExtension::Rb],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ruby3,
            syntax: concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "puts"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &[FileExtension::Php],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Php8,
            syntax: concat!(
                r#"<?php
echo ""#,
                output_for_hello_world!(),
                r#"\n";"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<?php"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Function, "echo"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ";"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &[FileExtension::Swift],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Swift5,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kotlin,
        extensions: &[FileExtension::Kt, FileExtension::Kts],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kotlin19,
            syntax: concat!(
                r#"fun main() {
    println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "fun"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "println"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scala,
        extensions: &[FileExtension::Scala, FileExtension::Sc],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scala3,
            syntax: concat!(
                r#"@main def hello(): Unit =
    println(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "@"),
                (TokenKind::Plain, "main "),
                (TokenKind::Keyword, "def"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "hello"),
                (TokenKind::Plain, "()"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Unit"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, "\n    println("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &[FileExtension::Pl, FileExtension::Pm],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perl5,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"\n";"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Plain, ";"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &[FileExtension::Lua],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lua54,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &[FileExtension::R],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::R4,
            syntax: concat!(r#"cat(""#, output_for_hello_world!(), r#"\n")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "cat"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &[FileExtension::Dart],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dart3,
            syntax: concat!(
                r"void main() {
    print('",
                output_for_hello_world!(),
                r"');
}"
            ),
            tokenized_syntax: &[
                (TokenKind::Type, "void"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "print"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elixir,
        extensions: &[FileExtension::Ex, FileExtension::Exs],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elixir1,
            syntax: concat!(r#"IO.puts(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Type, "IO"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "puts"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &[FileExtension::Erl, FileExtension::Hrl],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ErlangOtp,
            syntax: concat!(
                r#"-module(hello).
-export([main/0]).

main() ->
    io:format(""#,
                output_for_hello_world!(),
                r#"~n")."#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "-"),
                (TokenKind::Keyword, "module"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Type, "hello"),
                (TokenKind::Punctuation, ")."),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "-"),
                (TokenKind::Keyword, "export"),
                (TokenKind::Punctuation, "(["),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "/"),
                (TokenKind::Number, "0"),
                (TokenKind::Punctuation, "])."),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "->"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Type, "io"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Function, "format"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "~n\""),
                ),
                (TokenKind::Punctuation, ")."),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haskell,
        extensions: &[FileExtension::Hs, FileExtension::Lhs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haskell2010,
            syntax: concat!(
                r#"main :: IO ()
main = putStrLn ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Function, "main"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "::"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "IO"),
                (TokenKind::Plain, " ()\nmain "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, " putStrLn "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &[FileExtension::Clj, FileExtension::Cljs, FileExtension::Cljc],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clojure111,
            syntax: concat!(r#"(println ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Plain, "("),
                (TokenKind::Keyword, "println"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &[FileExtension::Fs, FileExtension::Fsx],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FSharp8,
            syntax: concat!(r#"printfn ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "printfn"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &[FileExtension::Ml, FileExtension::Mli],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OCaml5,
            syntax: concat!(
                r#"let () = print_endline ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "let"),
                (TokenKind::Plain, " () "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, " print_endline "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zig,
        extensions: &[FileExtension::Zig],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zig013,
            syntax: concat!(
                r#"const std = @import("std");

pub fn main() !void {
    try std.io.getStdOut().writer().print(""#,
                output_for_hello_world!(),
                r#"\n", .{});
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "const"),
                (TokenKind::Plain, " std "),
                (TokenKind::Punctuation, "="),
                (TokenKind::Plain, " "),
                (TokenKind::Macro, "@import"),
                (TokenKind::Punctuation, "("),
                (TokenKind::String, "\"std\""),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Keyword, "pub"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "fn"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "!"),
                (TokenKind::Type, "void"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "try"),
                (TokenKind::Plain, " std"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "io"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "getStdOut"),
                (TokenKind::Punctuation, "()."),
                (TokenKind::Function, "writer"),
                (TokenKind::Punctuation, "()."),
                (TokenKind::Function, "print"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ","),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, ".{});"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nim,
        extensions: &[FileExtension::Nim],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nim2,
            syntax: concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "echo"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Julia,
        extensions: &[FileExtension::Jl],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Julia110,
            syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "println"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &[FileExtension::Sh, FileExtension::Bash],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PosixShell,
            syntax: concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "echo"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &[FileExtension::Ps1, FileExtension::Psm1],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PowerShell7,
            syntax: concat!(r#"Write-Host ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "Write-Host"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
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
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortran,
        extensions: &[FileExtension::F, FileExtension::F90, FileExtension::F95],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortran90,
            syntax: concat!(
                r#"program hello
    print *, ""#,
                output_for_hello_world!(),
                r#""
end program hello"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " hello\n    "),
                (TokenKind::Keyword, "print"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "*,"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " hello"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobol,
        extensions: &[FileExtension::Cob, FileExtension::Cbl],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobol85,
            syntax: concat!(
                r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
           DISPLAY ""#,
                output_for_hello_world!(),
                r#"".
           STOP RUN."#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "       "),
                (TokenKind::Keyword, "IDENTIFICATION DIVISION"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "\n       "),
                (TokenKind::Keyword, "PROGRAM-ID"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, " HELLO"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "\n       "),
                (TokenKind::Keyword, "PROCEDURE DIVISION"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "\n           "),
                (TokenKind::Keyword, "DISPLAY"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "\n           "),
                (TokenKind::Keyword, "STOP RUN"),
                (TokenKind::Punctuation, "."),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ada,
        extensions: &[FileExtension::Adb, FileExtension::Ads],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ada2012,
            syntax: concat!(
                r#"with Ada.Text_IO; use Ada.Text_IO;

procedure Hello is
begin
    Put_Line (""#,
                output_for_hello_world!(),
                r#"");
end Hello;"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "with"),
                (TokenKind::Plain, " Ada"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "Text_IO"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "use"),
                (TokenKind::Plain, " Ada"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Plain, "Text_IO"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Keyword, "procedure"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "Hello"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "is"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "begin"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "Put_Line"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "Hello"),
                (TokenKind::Punctuation, ";"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pascal,
        extensions: &[FileExtension::Pas],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pascal,
            syntax: concat!(
                r"program Hello;
begin
    writeln('",
                output_for_hello_world!(),
                r"');
end."
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " Hello;\n"),
                (TokenKind::Keyword, "begin"),
                (TokenKind::Plain, "\n    writeln("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, ");\n"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Plain, "."),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic,
        extensions: &[FileExtension::Bas],
        categories: &[Scripting, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClassicBasic,
            syntax: concat!(
                r#"10 PRINT ""#,
                output_for_hello_world!(),
                r#""
20 END"#
            ),
            tokenized_syntax: &[
                (TokenKind::Number, "10"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "PRINT"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Number, "20"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "END"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &[FileExtension::Vb],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VbNet,
            syntax: concat!(
                r#"Module Program
    Sub Main()
        Console.WriteLine(""#,
                output_for_hello_world!(),
                r#"")
    End Sub
End Module"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "Module"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Program"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "Sub"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "Main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, "\n        "),
                (TokenKind::Type, "Console"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "WriteLine"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "End"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "Sub"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "End"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "Module"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groovy,
        extensions: &[FileExtension::Groovy, FileExtension::Gvy],
        categories: &[Scripting, Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Groovy4,
            syntax: concat!(r#"println ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "println"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &[FileExtension::M, FileExtension::Mm],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectiveC2,
            syntax: concat!(
                r#"#import <Foundation/Foundation.h>

int main() {
    NSLog(@""#,
                output_for_hello_world!(),
                r#"");
    return 0;
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#import"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "<Foundation/Foundation.h>"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "int"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "NSLog"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("@\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "return"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "0"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::D,
        extensions: &[FileExtension::D],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::D2,
            syntax: concat!(
                r#"import std.stdio;

void main() {
    writeln(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "import"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "std.stdio"),
                (TokenKind::Plain, ";\n\n"),
                (TokenKind::Type, "void"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Plain, "() {\n    writeln("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ");\n}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Crystal,
        extensions: &[FileExtension::Cr],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Crystal1,
            syntax: concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "puts"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &[FileExtension::Elm],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elm019,
            syntax: concat!(
                r#"module Main exposing (main)

import Html

main =
    Html.text ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "module"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Main"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "exposing"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "("),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Keyword, "import"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Html"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Function, "main"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "="),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Type, "Html"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "text"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Racket,
        extensions: &[FileExtension::Rkt],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Racket8,
            syntax: concat!(
                r#"#lang racket
(displayln ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#lang"),
                (TokenKind::Plain, " racket\n"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Function, "displayln"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CommonLisp,
        extensions: &[FileExtension::Lisp, FileExtension::Lsp, FileExtension::Cl],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AnsiCommonLisp,
            syntax: concat!(r#"(format t ""#, output_for_hello_world!(), r#"~%")"#),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "("),
                (TokenKind::Function, "format"),
                (TokenKind::Plain, " t "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "~%\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scheme,
        extensions: &[FileExtension::Scm, FileExtension::Ss],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::R7rs,
            syntax: concat!(
                r#"(display ""#,
                output_for_hello_world!(),
                r#"")
(newline)"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "("),
                (TokenKind::Plain, "display "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Plain, "newline"),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &[FileExtension::Pl, FileExtension::Pro],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IsoProlog,
            syntax: concat!(
                r":- initialization(main).

main :- write('",
                output_for_hello_world!(),
                r"'), nl, halt."
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, ":- initialization(main).\n\nmain :- "),
                (TokenKind::Function, "write"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, "), nl, halt."),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &[FileExtension::Sql],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sql2016,
            syntax: concat!(r"SELECT '", output_for_hello_world!(), r"' AS greeting;"),
            tokenized_syntax: &[
                (TokenKind::Keyword, "SELECT"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "AS"),
                (TokenKind::Plain, " greeting;"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &[FileExtension::M],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab,
            syntax: concat!(r"disp('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[
                (TokenKind::Keyword, "disp"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Solidity,
        extensions: &[FileExtension::Sol],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Solidity08,
            syntax: concat!(
                r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Hello {
    function greet() public pure returns (string memory) {
        return ""#,
                output_for_hello_world!(),
                r#"";
    }
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Comment, "// SPDX-License-Identifier: MIT"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "pragma"),
                (TokenKind::Plain, " solidity "),
                (TokenKind::Punctuation, "^"),
                (TokenKind::Number, "0.8.0"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Keyword, "contract"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Hello"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "function"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "greet"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "public"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "pure"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "returns"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "("),
                (TokenKind::Type, "string"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "memory"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n        "),
                (TokenKind::Keyword, "return"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Punctuation, "}"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Html,
        extensions: &[FileExtension::Html, FileExtension::Htm],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Html5,
            syntax: concat!(
                r"<!DOCTYPE html>
<html>
  <body>
    <p>",
                output_for_hello_world!(),
                r"</p>
  </body>
</html>"
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<!"),
                (TokenKind::Plain, "DOCTYPE html"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "html"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n  "),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "p"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "p"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n  "),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "html"),
                (TokenKind::Punctuation, ">"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Css,
        extensions: &[FileExtension::Css],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Css3,
            syntax: concat!(
                r#"body::before {
    content: ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, "::"),
                (TokenKind::Plain, "before "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Type, "content"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xml,
        extensions: &[FileExtension::Xml],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xml10,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>"#,
                output_for_hello_world!(),
                r#"</greeting>"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<?"),
                (TokenKind::Plain, "xml version"),
                (TokenKind::Punctuation, "="),
                (TokenKind::String, "\"1.0\""),
                (TokenKind::Plain, " encoding"),
                (TokenKind::Punctuation, "="),
                (TokenKind::String, "\"UTF-8\""),
                (TokenKind::Punctuation, "?>"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ">"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &[FileExtension::Md, FileExtension::Markdown],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CommonMark,
            syntax: concat!(r"# ", output_for_hello_world!()),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "#"),
                (TokenKind::Plain, concat!(" ", output_for_hello_world!())),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latex,
        extensions: &[FileExtension::Tex],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Latex2e,
            syntax: concat!(
                r"\documentclass{article}
\begin{document}
",
                output_for_hello_world!(),
                r"
\end{document}"
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "documentclass"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "article"),
                (TokenKind::Punctuation, "}"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "begin"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "document"),
                (TokenKind::Punctuation, "}"),
                (
                    TokenKind::Plain,
                    concat!("\n", output_for_hello_world!(), "\n"),
                ),
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "document"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainfuck,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainfuck,
            syntax: r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
            tokenized_syntax: &[
                (TokenKind::Number, "++++++++"),
                (TokenKind::Punctuation, "["),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "++++"),
                (TokenKind::Punctuation, "["),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "++"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "+++"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "+++"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "+"),
                (TokenKind::Keyword, "<<<<"),
                (TokenKind::Number, "-"),
                (TokenKind::Punctuation, "]"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "+"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "+"),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "-"),
                (TokenKind::Keyword, ">>"),
                (TokenKind::Number, "+"),
                (TokenKind::Punctuation, "["),
                (TokenKind::Keyword, "<"),
                (TokenKind::Punctuation, "]"),
                (TokenKind::Keyword, "<"),
                (TokenKind::Number, "-"),
                (TokenKind::Punctuation, "]"),
                (TokenKind::Keyword, ">>"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, ">"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Number, "---"),
                (TokenKind::Function, "."),
                (TokenKind::Number, "+++++++"),
                (TokenKind::Function, ".."),
                (TokenKind::Number, "+++"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, ">>"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, "<"),
                (TokenKind::Number, "-"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, "<"),
                (TokenKind::Function, "."),
                (TokenKind::Number, "+++"),
                (TokenKind::Function, "."),
                (TokenKind::Number, "------"),
                (TokenKind::Function, "."),
                (TokenKind::Number, "--------"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, ">>"),
                (TokenKind::Number, "+"),
                (TokenKind::Function, "."),
                (TokenKind::Keyword, ">"),
                (TokenKind::Number, "++"),
                (TokenKind::Function, "."),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &[FileExtension::Lol],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lolcode12,
            syntax: concat!(
                r#"HAI 1.2
    VISIBLE ""#,
                output_for_hello_world!(),
                r#""
KTHXBYE"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "HAI"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "1.2"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "VISIBLE"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "KTHXBYE"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Befunge,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Befunge93,
            syntax: r#""!dlroW ,olleH">:#,_@"#,
            tokenized_syntax: &[
                (TokenKind::String, "\"!dlroW ,olleH\""),
                (TokenKind::Keyword, ">"),
                (TokenKind::Function, ":"),
                (TokenKind::Keyword, "#"),
                (TokenKind::Function, ","),
                (TokenKind::Keyword, "_@"),
            ],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &[FileExtension::Apl],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apl,
            syntax: concat!(r"'", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::String,
                concat!("'", output_for_hello_world!(), "'"),
            )],
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol,
        extensions: &[FileExtension::Alg],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algol60,
            syntax: concat!(
                r#"begin
    outstring(1, ""#,
                output_for_hello_world!(),
                r#"\n")
end"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "begin"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "outstring"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Number, "1"),
                (TokenKind::Punctuation, ","),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "end"),
            ],
        }],
    },
];

/// Look up the hello-world snippet for a programming language.
///
/// The language is the explicit `language`, else inferred from `dialect`, else
/// from `extension`, else the default as the C programming language. `extension` only identifies or checks the
/// language; it does not change the snippet. If `dialect` is `None`, the
/// language's first recorded version is used.
///
/// Ambiguous extensions (e.g. `pl` for Perl and Prolog) resolve to the first
/// recorded language that uses them.
///
/// # Errors
///
/// Returns a [`ProgrammingLanguageError`] if the language, version or
/// extension is not recorded, or if `extension` does not belong to the language.
pub fn get_hello_world_syntax_for(
    language: Option<ProgrammingLanguageName>,
    dialect: Option<ProgrammingLanguageDialect>,
    extension: Option<FileExtension>,
) -> Result<&'static str, ProgrammingLanguageError> {
    get_hello_world_version_for(language, dialect, extension).map(|version| version.syntax)
}

/// Like [`get_hello_world_syntax_for`], but returns the whole resolved
/// [`LanguageVersion`], so callers also know which version was picked
/// (e.g. to look up its syntax-highlighting tokens).
///
/// # Errors
///
/// Same as [`get_hello_world_syntax_for`].
pub fn get_hello_world_version_for(
    language: Option<ProgrammingLanguageName>,
    dialect: Option<ProgrammingLanguageDialect>,
    extension: Option<FileExtension>,
) -> Result<&'static ProgrammingLanguageVersion, ProgrammingLanguageError> {
    let resolved_language = match (language, dialect, extension) {
        (Some(existing_language), _, _) => existing_language,
        (None, Some(existing_dialect), _) => {
            get_programming_language_name_from_dialect(existing_dialect)?
        }
        (None, None, Some(existing_extension)) => {
            get_programming_language_name_from_extension(existing_extension)?
        }
        (None, None, None) => DEFAULT_PROGRAMMING_LANGUAGE,
    };
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == resolved_language)
        .ok_or(LookupError::UnknownLanguage(resolved_language))?;

    if let Some(mismatched) = extension.filter(|ext| !lang.extensions.contains(ext)) {
        return Err(ProgrammingLanguageError::ExtensionMismatch {
            language: resolved_language,
            extension: mismatched,
        });
    }

    let version = match dialect {
        Some(wanted) => lang
            .versions
            .iter()
            .find(|version| version.label == wanted)
            .ok_or(LookupError::UnknownVariant {
                language: resolved_language,
                variant: wanted,
            })?,
        None => lang
            .versions
            .first()
            .ok_or(LookupError::NoVariants(resolved_language))?,
    };
    Ok(version)
}

/// Finds the first language that uses `extension`.
///
/// # Errors
///
/// Returns [`ProgrammingLanguageError::UnknownExtension`] if no recorded
/// language uses the extension.
pub fn get_programming_language_name_from_extension(
    extension: FileExtension,
) -> Result<ProgrammingLanguageName, ProgrammingLanguageError> {
    PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.extensions.contains(&extension))
        .map(|lang| lang.name)
        .ok_or(ProgrammingLanguageError::UnknownExtension(extension))
}

/// Finds the language that records the version `dialect`.
///
/// # Errors
///
/// Returns [`LookupError::NoOwner`] if no recorded language has the version.
pub fn get_programming_language_name_from_dialect(
    dialect: ProgrammingLanguageDialect,
) -> Result<ProgrammingLanguageName, ProgrammingLookupError> {
    PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.versions.iter().any(|d| d.label == dialect))
        .map(|lang| lang.name)
        .ok_or(LookupError::NoOwner(dialect))
}

/// Defaults to the first recorded extension of `language`.
///
/// # Errors
///
/// Returns [`LookupError::UnknownLanguage`] (wrapped) if the language is not
/// recorded, or [`ProgrammingLanguageError::NoExtensions`] if it has none.
pub fn get_default_extension_from_programming_language_name(
    language: ProgrammingLanguageName,
) -> Result<FileExtension, ProgrammingLanguageError> {
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(LookupError::UnknownLanguage(language))?;
    lang.extensions
        .first()
        .copied()
        .ok_or(ProgrammingLanguageError::NoExtensions(language))
}

/// Defaults to the first recorded version of `language`.
///
/// # Errors
///
/// Returns [`LookupError::UnknownLanguage`] if the language is not recorded,
/// or [`LookupError::NoVariants`] if it has no versions.
pub fn get_default_programming_language_dialect_from_language_name(
    language: ProgrammingLanguageName,
) -> Result<ProgrammingLanguageDialect, ProgrammingLookupError> {
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(LookupError::UnknownLanguage(language))?;
    lang.versions
        .first()
        .map(|version| version.label)
        .ok_or(LookupError::NoVariants(language))
}

#[must_use]
pub fn get_all_programming_language_names() -> Vec<ProgrammingLanguageName> {
    PROGRAMMING_LANGUAGES.iter().map(|lang| lang.name).collect()
}

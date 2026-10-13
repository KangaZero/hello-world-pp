use crate::data::common::DEFAULT_PROGRAMMING_LANGUAGE;

use super::common::ProgrammingLanguageCategory;
use super::lookup::LookupError;
use super::macros::name_enum;
use ProgrammingLanguageCategory::{Compiled, Esolang, Historical, Markup, Scripting};
use thiserror::Error;

/// Language/version resolution errors for the programming-language catalog.
pub type ProgrammingLookupError = LookupError<ProgrammingLanguageName, LanguageVersionDialect>;

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
    pub enum LanguageVersionDialect: ParseLanguageVersionDialectError("language version") {
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
pub struct ProgrammingLanguage {
    pub name: ProgrammingLanguageName,
    pub extensions: &'static [FileExtension],
    pub categories: &'static [ProgrammingLanguageCategory],
    pub versions: &'static [LanguageVersion],
}

#[derive(Debug, Clone, Copy)]
pub struct LanguageVersion {
    pub label: LanguageVersionDialect,
    pub syntax: &'static str, // the actual source snippet
}

pub static PROGRAMMING_LANGUAGES: &[ProgrammingLanguage] = &[
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C,
        extensions: &[FileExtension::C, FileExtension::H],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::C99,
            syntax: r#"#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
    return 0;
}"#,
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
            LanguageVersion {
                label: LanguageVersionDialect::Cpp98,
                syntax: r#"#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}"#,
            },
            LanguageVersion {
                label: LanguageVersionDialect::Cpp20,
                syntax: r#"#include <iostream>

int main() {
    std::cout << "Hello, World!\n";
}"#,
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSharp,
        extensions: &[FileExtension::Cs],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::CSharp10,
            syntax: r#"Console.WriteLine("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &[FileExtension::Java],
        categories: &[Compiled],
        versions: &[
            LanguageVersion {
                label: LanguageVersionDialect::Java17,
                syntax: r#"class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}"#,
            },
            LanguageVersion {
                label: LanguageVersionDialect::Java25CompactSourceFile,
                syntax: r#"void main() {
    IO.println("Hello, World!");
}"#,
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Python,
        extensions: &[FileExtension::Py],
        categories: &[Scripting],
        versions: &[
            LanguageVersion {
                label: LanguageVersionDialect::Python2,
                syntax: r#"print "Hello, World!""#,
            },
            LanguageVersion {
                label: LanguageVersionDialect::Python3,
                syntax: r#"print("Hello, World!")"#,
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaScript,
        extensions: &[FileExtension::Js, FileExtension::Mjs, FileExtension::Cjs],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::JavaScriptEs2020,
            syntax: r#"console.log("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TypeScript,
        extensions: &[FileExtension::Ts],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::TypeScript5,
            syntax: r#"const greeting: string = "Hello, World!";
console.log(greeting);"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rust,
        extensions: &[FileExtension::Rs],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Rust2021,
            syntax: r#"fn main() {
    println!("Hello, World!");
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Go,
        extensions: &[FileExtension::Go],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Go1,
            syntax: r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ruby,
        extensions: &[FileExtension::Rb],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Ruby3,
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &[FileExtension::Php],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Php8,
            syntax: r#"<?php
echo "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &[FileExtension::Swift],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Swift5,
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kotlin,
        extensions: &[FileExtension::Kt, FileExtension::Kts],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Kotlin19,
            syntax: r#"fun main() {
    println("Hello, World!")
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scala,
        extensions: &[FileExtension::Scala, FileExtension::Sc],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Scala3,
            syntax: r#"@main def hello(): Unit =
    println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &[FileExtension::Pl, FileExtension::Pm],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Perl5,
            syntax: r#"print "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &[FileExtension::Lua],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Lua54,
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &[FileExtension::R],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::R4,
            syntax: r#"cat("Hello, World!\n")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &[FileExtension::Dart],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Dart3,
            syntax: r"void main() {
    print('Hello, World!');
}",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elixir,
        extensions: &[FileExtension::Ex, FileExtension::Exs],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Elixir1,
            syntax: r#"IO.puts("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &[FileExtension::Erl, FileExtension::Hrl],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::ErlangOtp,
            syntax: r#"-module(hello).
-export([main/0]).

main() ->
    io:format("Hello, World!~n")."#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haskell,
        extensions: &[FileExtension::Hs, FileExtension::Lhs],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Haskell2010,
            syntax: r#"main :: IO ()
main = putStrLn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &[FileExtension::Clj, FileExtension::Cljs, FileExtension::Cljc],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Clojure111,
            syntax: r#"(println "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &[FileExtension::Fs, FileExtension::Fsx],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::FSharp8,
            syntax: r#"printfn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &[FileExtension::Ml, FileExtension::Mli],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::OCaml5,
            syntax: r#"let () = print_endline "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zig,
        extensions: &[FileExtension::Zig],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Zig013,
            syntax: r#"const std = @import("std");

pub fn main() !void {
    try std.io.getStdOut().writer().print("Hello, World!\n", .{});
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nim,
        extensions: &[FileExtension::Nim],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Nim2,
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Julia,
        extensions: &[FileExtension::Jl],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Julia110,
            syntax: r#"println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &[FileExtension::Sh, FileExtension::Bash],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::PosixShell,
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &[FileExtension::Ps1, FileExtension::Psm1],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::PowerShell7,
            syntax: r#"Write-Host "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembly,
        extensions: &[FileExtension::Asm, FileExtension::S],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::X8664LinuxSyscalls,
            syntax: r#"section .data
    msg db "Hello, World!", 0x0a
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
    syscall"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortran,
        extensions: &[FileExtension::F, FileExtension::F90, FileExtension::F95],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Fortran90,
            syntax: r#"program hello
    print *, "Hello, World!"
end program hello"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobol,
        extensions: &[FileExtension::Cob, FileExtension::Cbl],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Cobol85,
            syntax: r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
           DISPLAY "Hello, World!".
           STOP RUN."#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ada,
        extensions: &[FileExtension::Adb, FileExtension::Ads],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Ada2012,
            syntax: r#"with Ada.Text_IO; use Ada.Text_IO;

procedure Hello is
begin
    Put_Line ("Hello, World!");
end Hello;"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pascal,
        extensions: &[FileExtension::Pas],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Pascal,
            syntax: r"program Hello;
begin
    writeln('Hello, World!');
end.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic,
        extensions: &[FileExtension::Bas],
        categories: &[Scripting, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::ClassicBasic,
            syntax: r#"10 PRINT "Hello, World!"
20 END"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &[FileExtension::Vb],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::VbNet,
            syntax: r#"Module Program
    Sub Main()
        Console.WriteLine("Hello, World!")
    End Sub
End Module"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groovy,
        extensions: &[FileExtension::Groovy, FileExtension::Gvy],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Groovy4,
            syntax: r#"println "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &[FileExtension::M, FileExtension::Mm],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::ObjectiveC2,
            syntax: r#"#import <Foundation/Foundation.h>

int main() {
    NSLog(@"Hello, World!");
    return 0;
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::D,
        extensions: &[FileExtension::D],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::D2,
            syntax: r#"import std.stdio;

void main() {
    writeln("Hello, World!");
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Crystal,
        extensions: &[FileExtension::Cr],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Crystal1,
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &[FileExtension::Elm],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Elm019,
            syntax: r#"module Main exposing (main)

import Html

main =
    Html.text "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Racket,
        extensions: &[FileExtension::Rkt],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Racket8,
            syntax: r#"#lang racket
(displayln "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CommonLisp,
        extensions: &[FileExtension::Lisp, FileExtension::Lsp, FileExtension::Cl],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::AnsiCommonLisp,
            syntax: r#"(format t "Hello, World!~%")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scheme,
        extensions: &[FileExtension::Scm, FileExtension::Ss],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::R7rs,
            syntax: r#"(display "Hello, World!")
(newline)"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &[FileExtension::Pl, FileExtension::Pro],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::IsoProlog,
            syntax: r":- initialization(main).

main :- write('Hello, World!'), nl, halt.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &[FileExtension::Sql],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Sql2016,
            syntax: r"SELECT 'Hello, World!' AS greeting;",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &[FileExtension::M],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Matlab,
            syntax: r"disp('Hello, World!')",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Solidity,
        extensions: &[FileExtension::Sol],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Solidity08,
            syntax: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Hello {
    function greet() public pure returns (string memory) {
        return "Hello, World!";
    }
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Html,
        extensions: &[FileExtension::Html, FileExtension::Htm],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Html5,
            syntax: r"<!DOCTYPE html>
<html>
  <body>
    <p>Hello, World!</p>
  </body>
</html>",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Css,
        extensions: &[FileExtension::Css],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Css3,
            syntax: r#"body::before {
    content: "Hello, World!";
}"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xml,
        extensions: &[FileExtension::Xml],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Xml10,
            syntax: r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>Hello, World!</greeting>"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &[FileExtension::Md, FileExtension::Markdown],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::CommonMark,
            syntax: r"# Hello, World!",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latex,
        extensions: &[FileExtension::Tex],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Latex2e,
            syntax: r"\documentclass{article}
\begin{document}
Hello, World!
\end{document}",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainfuck,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Brainfuck,
            syntax: r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &[FileExtension::Lol],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Lolcode12,
            syntax: r#"HAI 1.2
    VISIBLE "Hello, World!"
KTHXBYE"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Befunge,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Befunge93,
            syntax: r#""!dlroW ,olleH">:#,_@"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &[FileExtension::Apl],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Apl,
            syntax: r"'Hello, World!'",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol,
        extensions: &[FileExtension::Alg],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Algol60,
            syntax: r#"begin
    outstring(1, "Hello, World!\n")
end"#,
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
    dialect: Option<LanguageVersionDialect>,
    extension: Option<FileExtension>,
) -> Result<&'static str, ProgrammingLanguageError> {
    let resolved_language = match (language, dialect, extension) {
        (Some(existing_language), _, _) => existing_language,
        (None, Some(existing_dialect), _) => {
            get_programming_language_from_dialect(existing_dialect)?
        }
        (None, None, Some(existing_extension)) => {
            get_programming_language_from_extension(existing_extension)?
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
    Ok(version.syntax)
}

/// Finds the first language that uses `extension`.
///
/// # Errors
///
/// Returns [`ProgrammingLanguageError::UnknownExtension`] if no recorded
/// language uses the extension.
pub fn get_programming_language_from_extension(
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
pub fn get_programming_language_from_dialect(
    dialect: LanguageVersionDialect,
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
) -> Result<LanguageVersionDialect, ProgrammingLookupError> {
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

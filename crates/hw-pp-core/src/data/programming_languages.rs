use super::common::ProgrammingLanguageCategory;
use ProgrammingLanguageCategory::{Compiled, Esolang, Historical, Markup, Scripting};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ProgrammingLanguageError {
    #[error("no greetings found for language {0}: not a recorded language")]
    UnknownLanguage(ProgrammingLanguageName),

    #[error(
        "no greetings found for {language}: dialect {dialect} is not recorded for this language"
    )]
    UnknownDialect {
        language: ProgrammingLanguageName,
        dialect: LanguageVersionDialect,
    },

    /// The dialect has no recorded greetings.
    #[error("no greeting recorded for {language} ({dialect})")]
    NoGreeting {
        /// The requested language.
        language: ProgrammingLanguageName,
        /// The requested dialect.
        dialect: LanguageVersionDialect,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProgrammingLanguageName {
    #[default]
    C,
    Cpp,
    CSharp,
    Java,
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Go,
    Ruby,
    Php,
    Swift,
    Kotlin,
    Scala,
    Perl,
    Lua,
    R,
    Dart,
    Elixir,
    Erlang,
    Haskell,
    Clojure,
    FSharp,
    OCaml,
    Zig,
    Nim,
    Julia,
    Bash,
    PowerShell,
    Assembly,
    Fortran,
    Cobol,
    Ada,
    Pascal,
    Basic,
    VisualBasic,
    Groovy,
    ObjectiveC,
    D,
    Crystal,
    Elm,
    Racket,
    CommonLisp,
    Scheme,
    Prolog,
    Sql,
    Matlab,
    Solidity,
    Html,
    Css,
    Xml,
    Markdown,
    Latex,
    Brainfuck,
    Lolcode,
    Befunge,
    Apl,
    Algol,
}

impl ProgrammingLanguageName {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::C => "C",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::Java => "Java",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Rust => "Rust",
            Self::Go => "Go",
            Self::Ruby => "Ruby",
            Self::Php => "PHP",
            Self::Swift => "Swift",
            Self::Kotlin => "Kotlin",
            Self::Scala => "Scala",
            Self::Perl => "Perl",
            Self::Lua => "Lua",
            Self::R => "R",
            Self::Dart => "Dart",
            Self::Elixir => "Elixir",
            Self::Erlang => "Erlang",
            Self::Haskell => "Haskell",
            Self::Clojure => "Clojure",
            Self::FSharp => "F#",
            Self::OCaml => "OCaml",
            Self::Zig => "Zig",
            Self::Nim => "Nim",
            Self::Julia => "Julia",
            Self::Bash => "Bash",
            Self::PowerShell => "PowerShell",
            Self::Assembly => "Assembly",
            Self::Fortran => "Fortran",
            Self::Cobol => "COBOL",
            Self::Ada => "Ada",
            Self::Pascal => "Pascal",
            Self::Basic => "BASIC",
            Self::VisualBasic => "Visual Basic",
            Self::Groovy => "Groovy",
            Self::ObjectiveC => "Objective-C",
            Self::D => "D",
            Self::Crystal => "Crystal",
            Self::Elm => "Elm",
            Self::Racket => "Racket",
            Self::CommonLisp => "Common Lisp",
            Self::Scheme => "Scheme",
            Self::Prolog => "Prolog",
            Self::Sql => "SQL",
            Self::Matlab => "MATLAB",
            Self::Solidity => "Solidity",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Xml => "XML",
            Self::Markdown => "Markdown",
            Self::Latex => "LaTeX",
            Self::Brainfuck => "Brainfuck",
            Self::Lolcode => "LOLCODE",
            Self::Befunge => "Befunge",
            Self::Apl => "APL",
            Self::Algol => "ALGOL",
        }
    }
}

impl core::fmt::Display for ProgrammingLanguageName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageVersionDialect {
    C99,
    Cpp98,
    Cpp20,
    CSharp10,
    Java17,
    Java25CompactSourceFile,
    Python2,
    Python3,
    JavaScriptEs2020,
    TypeScript5,
    Rust2021,
    Go1,
    Ruby3,
    Php8,
    Swift5,
    Kotlin19,
    Scala3,
    Perl5,
    Lua54,
    R4,
    Dart3,
    Elixir1,
    ErlangOtp,
    Haskell2010,
    Clojure111,
    FSharp8,
    OCaml5,
    Zig013,
    Nim2,
    Julia110,
    PosixShell,
    PowerShell7,
    X8664LinuxSyscalls,
    Fortran90,
    Cobol85,
    Ada2012,
    Pascal,
    ClassicBasic,
    VbNet,
    Groovy4,
    ObjectiveC2,
    D2,
    Crystal1,
    Elm019,
    Racket8,
    AnsiCommonLisp,
    R7rs,
    IsoProlog,
    Sql2016,
    Matlab,
    Solidity08,
    Html5,
    Css3,
    Xml10,
    CommonMark,
    Latex2e,
    Brainfuck,
    Lolcode12,
    Befunge93,
    Apl,
    Algol60,
}

impl LanguageVersionDialect {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::C99 => "C99",
            Self::Cpp98 => "C++98",
            Self::Cpp20 => "C++20",
            Self::CSharp10 => "C# 10",
            Self::Java17 => "Java 17",
            Self::Java25CompactSourceFile => "Java 25 (compact source file)",
            Self::Python2 => "Python 2",
            Self::Python3 => "Python 3",
            Self::JavaScriptEs2020 => "ES2020",
            Self::TypeScript5 => "TypeScript 5",
            Self::Rust2021 => "Rust 2021",
            Self::Go1 => "Go 1",
            Self::Ruby3 => "Ruby 3",
            Self::Php8 => "PHP 8",
            Self::Swift5 => "Swift 5",
            Self::Kotlin19 => "Kotlin 1.9",
            Self::Scala3 => "Scala 3",
            Self::Perl5 => "Perl 5",
            Self::Lua54 => "Lua 5.4",
            Self::R4 => "R 4",
            Self::Dart3 => "Dart 3",
            Self::Elixir1 => "Elixir 1",
            Self::ErlangOtp => "Erlang/OTP",
            Self::Haskell2010 => "Haskell 2010",
            Self::Clojure111 => "Clojure 1.11",
            Self::FSharp8 => "F# 8",
            Self::OCaml5 => "OCaml 5",
            Self::Zig013 => "Zig 0.13",
            Self::Nim2 => "Nim 2",
            Self::Julia110 => "Julia 1.10",
            Self::PosixShell => "POSIX shell",
            Self::PowerShell7 => "PowerShell 7",
            Self::X8664LinuxSyscalls => "x86-64 (Linux syscalls)",
            Self::Fortran90 => "Fortran 90",
            Self::Cobol85 => "COBOL 85",
            Self::Ada2012 => "Ada 2012",
            Self::Pascal => "Pascal",
            Self::ClassicBasic => "Classic BASIC",
            Self::VbNet => "VB.NET",
            Self::Groovy4 => "Groovy 4",
            Self::ObjectiveC2 => "Objective-C 2.0",
            Self::D2 => "D 2",
            Self::Crystal1 => "Crystal 1",
            Self::Elm019 => "Elm 0.19",
            Self::Racket8 => "Racket 8",
            Self::AnsiCommonLisp => "ANSI Common Lisp",
            Self::R7rs => "R7RS",
            Self::IsoProlog => "ISO Prolog",
            Self::Sql2016 => "SQL:2016",
            Self::Matlab => "MATLAB",
            Self::Solidity08 => "Solidity 0.8",
            Self::Html5 => "HTML5",
            Self::Css3 => "CSS3",
            Self::Xml10 => "XML 1.0",
            Self::CommonMark => "CommonMark",
            Self::Latex2e => "LaTeX2e",
            Self::Brainfuck => "Brainfuck",
            Self::Lolcode12 => "LOLCODE 1.2",
            Self::Befunge93 => "Befunge-93",
            Self::Apl => "APL",
            Self::Algol60 => "ALGOL 60",
        }
    }
}

impl core::fmt::Display for LanguageVersionDialect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguage {
    pub name: ProgrammingLanguageName,
    pub extensions: &'static [&'static str],
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
        extensions: &["c", "h"],
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
        extensions: &["cpp", "cc", "cxx", "hpp"],
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
        extensions: &["cs"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::CSharp10,
            syntax: r#"Console.WriteLine("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &["java"],
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
        extensions: &["py"],
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
        extensions: &["js", "mjs", "cjs"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::JavaScriptEs2020,
            syntax: r#"console.log("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TypeScript,
        extensions: &["ts"],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::TypeScript5,
            syntax: r#"const greeting: string = "Hello, World!";
console.log(greeting);"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rust,
        extensions: &["rs"],
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
        extensions: &["go"],
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
        extensions: &["rb"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Ruby3,
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &["php"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Php8,
            syntax: r#"<?php
echo "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &["swift"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Swift5,
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kotlin,
        extensions: &["kt", "kts"],
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
        extensions: &["scala", "sc"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Scala3,
            syntax: r#"@main def hello(): Unit =
    println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &["pl", "pm"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Perl5,
            syntax: r#"print "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &["lua"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Lua54,
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &["r", "R"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::R4,
            syntax: r#"cat("Hello, World!\n")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &["dart"],
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
        extensions: &["ex", "exs"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Elixir1,
            syntax: r#"IO.puts("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &["erl", "hrl"],
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
        extensions: &["hs", "lhs"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Haskell2010,
            syntax: r#"main :: IO ()
main = putStrLn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &["clj", "cljs", "cljc"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Clojure111,
            syntax: r#"(println "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &["fs", "fsx"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::FSharp8,
            syntax: r#"printfn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &["ml", "mli"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::OCaml5,
            syntax: r#"let () = print_endline "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zig,
        extensions: &["zig"],
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
        extensions: &["nim"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Nim2,
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Julia,
        extensions: &["jl"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Julia110,
            syntax: r#"println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &["sh", "bash"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::PosixShell,
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &["ps1", "psm1"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::PowerShell7,
            syntax: r#"Write-Host "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembly,
        extensions: &["asm", "s"],
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
        extensions: &["f", "f90", "f95"],
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
        extensions: &["cob", "cbl"],
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
        extensions: &["adb", "ads"],
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
        extensions: &["pas"],
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
        extensions: &["bas"],
        categories: &[Scripting, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::ClassicBasic,
            syntax: r#"10 PRINT "Hello, World!"
20 END"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &["vb"],
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
        extensions: &["groovy", "gvy"],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Groovy4,
            syntax: r#"println "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &["m", "mm"],
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
        extensions: &["d"],
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
        extensions: &["cr"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Crystal1,
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &["elm"],
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
        extensions: &["rkt"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Racket8,
            syntax: r#"#lang racket
(displayln "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CommonLisp,
        extensions: &["lisp", "lsp", "cl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::AnsiCommonLisp,
            syntax: r#"(format t "Hello, World!~%")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scheme,
        extensions: &["scm", "ss"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::R7rs,
            syntax: r#"(display "Hello, World!")
(newline)"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &["pl", "pro"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::IsoProlog,
            syntax: r":- initialization(main).

main :- write('Hello, World!'), nl, halt.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &["sql"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Sql2016,
            syntax: r"SELECT 'Hello, World!' AS greeting;",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &["m"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Matlab,
            syntax: r"disp('Hello, World!')",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Solidity,
        extensions: &["sol"],
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
        extensions: &["html", "htm"],
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
        extensions: &["css"],
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
        extensions: &["xml"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Xml10,
            syntax: r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>Hello, World!</greeting>"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &["md", "markdown"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::CommonMark,
            syntax: r"# Hello, World!",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latex,
        extensions: &["tex"],
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
        extensions: &["bf"],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Brainfuck,
            syntax: r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &["lol"],
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
        extensions: &["bf"],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Befunge93,
            syntax: r#""!dlroW ,olleH">:#,_@"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &["apl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Apl,
            syntax: r"'Hello, World!'",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol,
        extensions: &["alg"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: LanguageVersionDialect::Algol60,
            syntax: r#"begin
    outstring(1, "Hello, World!\n")
end"#,
        }],
    },
];

pub fn hello_world_syntax_for(
    _language: Option<ProgrammingLanguage>,
    _dialect: Option<LanguageVersionDialect>,
) {
}

#[must_use]
pub fn get_all_programming_language_names() -> Vec<ProgrammingLanguageName> {
    PROGRAMMING_LANGUAGES.iter().map(|lang| lang.name).collect()
}

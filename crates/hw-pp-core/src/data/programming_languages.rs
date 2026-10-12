use super::common::ProgrammingLanguageCategory;
use ProgrammingLanguageCategory::{Compiled, Esolang, Historical, Markup, Scripting};

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

#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguage {
    pub name: ProgrammingLanguageName,
    pub extensions: &'static [&'static str],
    pub categories: &'static [ProgrammingLanguageCategory],
    pub versions: &'static [LanguageVersion],
}

#[derive(Debug, Clone, Copy)]
pub struct LanguageVersion {
    pub label: &'static str,  // "C++98", "C++20", "Python 3", etc.
    pub syntax: &'static str, // the actual source snippet
}

pub static PROGRAMMING_LANGUAGES: &[ProgrammingLanguage] = &[
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C,
        extensions: &["c", "h"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "C99",
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
                label: "C++98",
                syntax: r#"#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}"#,
            },
            LanguageVersion {
                label: "C++20",
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
            label: "C# 10",
            syntax: r#"Console.WriteLine("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &["java"],
        categories: &[Compiled],
        versions: &[
            LanguageVersion {
                label: "Java 17",
                syntax: r#"class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}"#,
            },
            LanguageVersion {
                label: "Java 25 (compact source file)",
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
                label: "Python 2",
                syntax: r#"print "Hello, World!""#,
            },
            LanguageVersion {
                label: "Python 3",
                syntax: r#"print("Hello, World!")"#,
            },
        ],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaScript,
        extensions: &["js", "mjs", "cjs"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "ES2020",
            syntax: r#"console.log("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TypeScript,
        extensions: &["ts"],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: "TypeScript 5",
            syntax: r#"const greeting: string = "Hello, World!";
console.log(greeting);"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rust,
        extensions: &["rs"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Rust 2021",
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
            label: "Go 1",
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
            label: "Ruby 3",
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &["php"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "PHP 8",
            syntax: r#"<?php
echo "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &["swift"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Swift 5",
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kotlin,
        extensions: &["kt", "kts"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Kotlin 1.9",
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
            label: "Scala 3",
            syntax: r#"@main def hello(): Unit =
    println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &["pl", "pm"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "Perl 5",
            syntax: r#"print "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &["lua"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "Lua 5.4",
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &["r", "R"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "R 4",
            syntax: r#"cat("Hello, World!\n")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &["dart"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Dart 3",
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
            label: "Elixir 1",
            syntax: r#"IO.puts("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &["erl", "hrl"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Erlang/OTP",
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
            label: "Haskell 2010",
            syntax: r#"main :: IO ()
main = putStrLn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &["clj", "cljs", "cljc"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Clojure 1.11",
            syntax: r#"(println "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FSharp,
        extensions: &["fs", "fsx"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "F# 8",
            syntax: r#"printfn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &["ml", "mli"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "OCaml 5",
            syntax: r#"let () = print_endline "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zig,
        extensions: &["zig"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Zig 0.13",
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
            label: "Nim 2",
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Julia,
        extensions: &["jl"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Julia 1.10",
            syntax: r#"println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &["sh", "bash"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "POSIX shell",
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &["ps1", "psm1"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "PowerShell 7",
            syntax: r#"Write-Host "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembly,
        extensions: &["asm", "s"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "x86-64 (Linux syscalls)",
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
            label: "Fortran 90",
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
            label: "COBOL 85",
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
            label: "Ada 2012",
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
            label: "Pascal",
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
            label: "Classic BASIC",
            syntax: r#"10 PRINT "Hello, World!"
20 END"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &["vb"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "VB.NET",
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
            label: "Groovy 4",
            syntax: r#"println "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &["m", "mm"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "Objective-C 2.0",
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
            label: "D 2",
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
            label: "Crystal 1",
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &["elm"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Elm 0.19",
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
            label: "Racket 8",
            syntax: r#"#lang racket
(displayln "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CommonLisp,
        extensions: &["lisp", "lsp", "cl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "ANSI Common Lisp",
            syntax: r#"(format t "Hello, World!~%")"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scheme,
        extensions: &["scm", "ss"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "R7RS",
            syntax: r#"(display "Hello, World!")
(newline)"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &["pl", "pro"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "ISO Prolog",
            syntax: r":- initialization(main).

main :- write('Hello, World!'), nl, halt.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &["sql"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "SQL:2016",
            syntax: r"SELECT 'Hello, World!' AS greeting;",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &["m"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "MATLAB",
            syntax: r"disp('Hello, World!')",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Solidity,
        extensions: &["sol"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Solidity 0.8",
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
            label: "HTML5",
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
            label: "CSS3",
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
            label: "XML 1.0",
            syntax: r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>Hello, World!</greeting>"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &["md", "markdown"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: "CommonMark",
            syntax: r"# Hello, World!",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latex,
        extensions: &["tex"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: "LaTeX2e",
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
            label: "Brainfuck",
            syntax: r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &["lol"],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: "LOLCODE 1.2",
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
            label: "Befunge-93",
            syntax: r#""!dlroW ,olleH">:#,_@"#,
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &["apl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "APL",
            syntax: r"'Hello, World!'",
        }],
    },
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol,
        extensions: &["alg"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "ALGOL 60",
            syntax: r#"begin
    outstring(1, "Hello, World!\n")
end"#,
        }],
    },
];

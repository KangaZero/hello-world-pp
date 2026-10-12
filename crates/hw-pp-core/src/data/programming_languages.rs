use super::common::ProgrammingLanguageCategory;
use ProgrammingLanguageCategory::{Compiled, Esolang, Historical, Markup, Scripting};

#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguage {
    pub name: &'static str,
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
        name: "C",
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
        name: "C++",
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
        name: "C#",
        extensions: &["cs"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "C# 10",
            syntax: r#"Console.WriteLine("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: "Java",
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
        name: "Python",
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
        name: "JavaScript",
        extensions: &["js", "mjs", "cjs"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "ES2020",
            syntax: r#"console.log("Hello, World!");"#,
        }],
    },
    ProgrammingLanguage {
        name: "TypeScript",
        extensions: &["ts"],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: "TypeScript 5",
            syntax: r#"const greeting: string = "Hello, World!";
console.log(greeting);"#,
        }],
    },
    ProgrammingLanguage {
        name: "Rust",
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
        name: "Go",
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
        name: "Ruby",
        extensions: &["rb"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "Ruby 3",
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "PHP",
        extensions: &["php"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "PHP 8",
            syntax: r#"<?php
echo "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: "Swift",
        extensions: &["swift"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Swift 5",
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Kotlin",
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
        name: "Scala",
        extensions: &["scala", "sc"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Scala 3",
            syntax: r#"@main def hello(): Unit =
    println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Perl",
        extensions: &["pl", "pm"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "Perl 5",
            syntax: r#"print "Hello, World!\n";"#,
        }],
    },
    ProgrammingLanguage {
        name: "Lua",
        extensions: &["lua"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "Lua 5.4",
            syntax: r#"print("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "R",
        extensions: &["r", "R"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "R 4",
            syntax: r#"cat("Hello, World!\n")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Dart",
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
        name: "Elixir",
        extensions: &["ex", "exs"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Elixir 1",
            syntax: r#"IO.puts("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Erlang",
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
        name: "Haskell",
        extensions: &["hs", "lhs"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Haskell 2010",
            syntax: r#"main :: IO ()
main = putStrLn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Clojure",
        extensions: &["clj", "cljs", "cljc"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Clojure 1.11",
            syntax: r#"(println "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "F#",
        extensions: &["fs", "fsx"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "F# 8",
            syntax: r#"printfn "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "OCaml",
        extensions: &["ml", "mli"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "OCaml 5",
            syntax: r#"let () = print_endline "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Zig",
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
        name: "Nim",
        extensions: &["nim"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Nim 2",
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Julia",
        extensions: &["jl"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Julia 1.10",
            syntax: r#"println("Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Bash",
        extensions: &["sh", "bash"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "POSIX shell",
            syntax: r#"echo "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "PowerShell",
        extensions: &["ps1", "psm1"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "PowerShell 7",
            syntax: r#"Write-Host "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Assembly",
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
        name: "Fortran",
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
        name: "COBOL",
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
        name: "Ada",
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
        name: "Pascal",
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
        name: "BASIC",
        extensions: &["bas"],
        categories: &[Scripting, Historical],
        versions: &[LanguageVersion {
            label: "Classic BASIC",
            syntax: r#"10 PRINT "Hello, World!"
20 END"#,
        }],
    },
    ProgrammingLanguage {
        name: "Visual Basic",
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
        name: "Groovy",
        extensions: &["groovy", "gvy"],
        categories: &[Scripting, Compiled],
        versions: &[LanguageVersion {
            label: "Groovy 4",
            syntax: r#"println "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Objective-C",
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
        name: "D",
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
        name: "Crystal",
        extensions: &["cr"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "Crystal 1",
            syntax: r#"puts "Hello, World!""#,
        }],
    },
    ProgrammingLanguage {
        name: "Elm",
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
        name: "Racket",
        extensions: &["rkt"],
        categories: &[Compiled, Scripting],
        versions: &[LanguageVersion {
            label: "Racket 8",
            syntax: r#"#lang racket
(displayln "Hello, World!")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Common Lisp",
        extensions: &["lisp", "lsp", "cl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "ANSI Common Lisp",
            syntax: r#"(format t "Hello, World!~%")"#,
        }],
    },
    ProgrammingLanguage {
        name: "Scheme",
        extensions: &["scm", "ss"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "R7RS",
            syntax: r#"(display "Hello, World!")
(newline)"#,
        }],
    },
    ProgrammingLanguage {
        name: "Prolog",
        extensions: &["pl", "pro"],
        categories: &[Compiled],
        versions: &[LanguageVersion {
            label: "ISO Prolog",
            syntax: r":- initialization(main).

main :- write('Hello, World!'), nl, halt.",
        }],
    },
    ProgrammingLanguage {
        name: "SQL",
        extensions: &["sql"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "SQL:2016",
            syntax: r"SELECT 'Hello, World!' AS greeting;",
        }],
    },
    ProgrammingLanguage {
        name: "MATLAB",
        extensions: &["m"],
        categories: &[Scripting],
        versions: &[LanguageVersion {
            label: "MATLAB",
            syntax: r"disp('Hello, World!')",
        }],
    },
    ProgrammingLanguage {
        name: "Solidity",
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
        name: "HTML",
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
        name: "CSS",
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
        name: "XML",
        extensions: &["xml"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: "XML 1.0",
            syntax: r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>Hello, World!</greeting>"#,
        }],
    },
    ProgrammingLanguage {
        name: "Markdown",
        extensions: &["md", "markdown"],
        categories: &[Markup],
        versions: &[LanguageVersion {
            label: "CommonMark",
            syntax: r"# Hello, World!",
        }],
    },
    ProgrammingLanguage {
        name: "LaTeX",
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
        name: "Brainfuck",
        extensions: &["bf"],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: "Brainfuck",
            syntax: r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
        }],
    },
    ProgrammingLanguage {
        name: "LOLCODE",
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
        name: "Befunge",
        extensions: &["bf"],
        categories: &[Esolang],
        versions: &[LanguageVersion {
            label: "Befunge-93",
            syntax: r#""!dlroW ,olleH">:#,_@"#,
        }],
    },
    ProgrammingLanguage {
        name: "APL",
        extensions: &["apl"],
        categories: &[Compiled, Historical],
        versions: &[LanguageVersion {
            label: "APL",
            syntax: r"'Hello, World!'",
        }],
    },
    ProgrammingLanguage {
        name: "ALGOL",
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

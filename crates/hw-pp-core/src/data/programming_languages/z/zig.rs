//! Hello-world snippets for `ProgrammingLanguageName::Zig`.
//!
//! Sources:
//! - <https://ziglang.org/documentation/0.13.0/#Hello-World>
//! - <https://ziglang.org/learn/getting-started/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zig => "Zig",
    versions: [Zig013 => "Zig 0.13"],
    extensions: [Zig => "zig"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
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
    };

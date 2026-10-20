//! Hello-world snippet for `ProgrammingLanguageName::Systemverilog`.
//!
//! Imported from leachim6/hello-world (PR #1570 `s/SystemVerilog.sv`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1570>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::HardwareDescription,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Systemverilog => "SystemVerilog",
    versions: [Systemverilog => "SystemVerilog"],
    extensions: [Sv => "sv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Systemverilog,
        extensions: &[FileExtension::Sv],
        categories: &[HardwareDescription],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Systemverilog,
            syntax: concat!(
                r#"module main();
    initial begin
        $display(""#,
                output_for_hello_world!(),
                r#"");
    end
endmodule"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"module"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"initial"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"begin"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Function, r"$display"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"end"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"endmodule"),
            ],
        }],
    };

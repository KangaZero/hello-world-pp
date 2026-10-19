//! Hello-world snippets for `ProgrammingLanguageName::CommonLisp`.
//!
//! Sources:
//! - <https://lisp-lang.org/learn/getting-started/>
//! - <https://www.lispworks.com/documentation/HyperSpec/Body/f_format.htm>

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
    language: CommonLisp => "Common Lisp",
    versions: [AnsiCommonLisp => "ANSI Common Lisp"],
    extensions: [Lisp => "lisp", Lsp => "lsp", Cl => "cl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
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
    };

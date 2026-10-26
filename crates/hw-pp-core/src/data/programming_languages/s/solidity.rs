//! Hello-world snippet for `ProgrammingLanguageName::Solidity`.
//!
//! Sources:
//! - <https://docs.soliditylang.org/en/latest/introduction-to-smart-contracts.html>
//! - <https://docs.soliditylang.org/en/latest/layout-of-source-files.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Solidity => "Solidity",
    versions: [Solidity08 => "Solidity 0.8"],
    extensions: [Sol => "sol"],
}

const SOLIDITY08: &str = concat!(
    r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Hello {
    function greet() public pure returns (string memory) {
        return ""#,
    output_for_hello_world!(),
    r#"";
    }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Solidity,
        extensions: &[FileExtension::Sol],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Solidity08,
            syntax: SOLIDITY08,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SOLIDITY08,
                    [
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
                )
            },
        }],
    };

//! Hello-world snippets for `ProgrammingLanguageName::Cpp`.
//!
//! Sources:
//! - <https://en.cppreference.com/cpp/io/cout>
//! - <https://en.cppreference.com/cpp/language/main_function>
//! - <https://en.cppreference.com/cpp/20>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cpp => "C++",
    versions: [Cpp98 => "C++98", Cpp20 => "C++20"],
    extensions: [Cpp => "cpp", Cc => "cc", Cxx => "cxx", Hpp => "hpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
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
    };

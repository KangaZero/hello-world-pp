//! Hello-world snippet for `ProgrammingLanguageName::Julia`.
//!
//! Sources:
//! - <https://docs.julialang.org/en/v1/manual/getting-started/>
//! - <https://docs.julialang.org/en/v1/base/io-network/#Base.println>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Julia => "Julia",
    versions: [Julia110 => "Julia 1.10"],
    extensions: [Jl => "jl"],
}

const JULIA110: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Julia,
        extensions: &[FileExtension::Jl],
        categories: &[Compiled, Scripting],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Julia110,
            syntax: JULIA110,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JULIA110,
                    [
                        (TokenKind::Function, "println"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippets for `ProgrammingLanguageName::Ruby`.
//!
//! Sources:
//! - <https://www.ruby-lang.org/en/documentation/quickstart/>
//! - <https://docs.ruby-lang.org/en/master/Kernel.html#method-i-puts>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ruby, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ruby => "Ruby",
    versions: [Ruby3 => "Ruby 3"],
    extensions: [Rb => "rb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ruby,
        extensions: &[FileExtension::Rb],
        categories: &[Scripting],
        families: &[Ruby],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ruby3,
            syntax: concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "puts"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };

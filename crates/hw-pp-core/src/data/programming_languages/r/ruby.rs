//! Hello-world snippet for `ProgrammingLanguageName::Ruby`.
//!
//! Sources:
//! - <https://www.ruby-lang.org/en/documentation/quickstart/>
//! - <https://docs.ruby-lang.org/en/master/Kernel.html#method-i-puts>
//! - <https://www.wikidata.org/wiki/Q161053> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ruby, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ruby => "Ruby",
    versions: [Ruby3 => "Ruby 3"],
    extensions: [Rb => "rb"],
}

const RUBY3: &str = concat!(r#"puts ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ruby,
        extensions: &[FileExtension::Rb],
        categories: &[Scripting],
        families: &[Ruby],
        paradigms: &[Imperative, ObjectOriented, Functional, Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ruby3,
            syntax: RUBY3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RUBY3,
                    [
                        (TokenKind::Function, "puts"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };

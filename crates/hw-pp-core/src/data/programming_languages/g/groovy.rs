//! Hello-world snippet for `ProgrammingLanguageName::Groovy`.
//!
//! Sources:
//! - <https://groovy-lang.org/semantics.html>
//! - <https://groovy-lang.org/differences.html>
//! - <https://groovy-lang.org/releasenotes/groovy-4.0.html>
//! - <https://en.wikipedia.org/wiki/Apache_Groovy> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    AspectOriented, Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Groovy => "Groovy",
    versions: [Groovy4 => "Groovy 4"],
    extensions: [Groovy => "groovy", Gvy => "gvy"],
}

const GROOVY4: &str = concat!(r#"println ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groovy,
        extensions: &[FileExtension::Groovy, FileExtension::Gvy],
        categories: &[Scripting, Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, AspectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Groovy4,
            syntax: GROOVY4,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GROOVY4,
                    [
                        (TokenKind::Function, "println"),
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

//! Hello-world snippet for `ProgrammingLanguageName::Frink`.
//!
//! Imported from leachim6/hello-world (`f/Frink.frink` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Frink.frink>
//! - <https://en.wikipedia.org/wiki/Frink_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Frink => "Frink",
    versions: [Frink => "Frink"],
    extensions: [Frink => "frink"],
}

const FRINK: &str = concat!(r#"println[""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Frink,
        extensions: &[FileExtension::Frink],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Frink,
            syntax: FRINK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FRINK,
                    [
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"["),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"]"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Potigol`.
//!
//! Imported from leachim6/hello-world (`p/Potigol.poti` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Potigol.poti>
//! - <https://potigol.github.io/docs/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Potigol => "Potigol",
    versions: [Potigol => "Potigol"],
    extensions: [Poti => "poti"],
}

const POTIGOL: &str = concat!(r#"escreva ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Potigol,
        extensions: &[FileExtension::Poti],
        categories: &[Educational],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Potigol,
            syntax: POTIGOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POTIGOL,
                    [
                        (TokenKind::Plain, r"escreva "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };

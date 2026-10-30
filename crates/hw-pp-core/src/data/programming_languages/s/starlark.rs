//! Hello-world snippet for `ProgrammingLanguageName::Starlark`.
//!
//! Imported from leachim6/hello-world (`s/Starlark.star` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Starlark.star>
//! - <https://www.wikidata.org/wiki/Q104094003> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Starlark => "Starlark",
    versions: [Starlark => "Starlark"],
    extensions: [Star => "star"],
}

const STARLARK: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Starlark,
        extensions: &[FileExtension::Star],
        categories: &[Configuration],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Starlark,
            syntax: STARLARK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STARLARK,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Shen`.
//!
//! Imported from leachim6/hello-world (`s/Shen.shen` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Shen.shen>
//! - <https://shenlanguage.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Logic};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Shen => "Shen",
    versions: [Shen => "Shen"],
    extensions: [Shen => "shen"],
}

const SHEN: &str = concat!(r#"(pr ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Shen,
        extensions: &[FileExtension::Shen],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Functional, Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Shen,
            syntax: SHEN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SHEN,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"pr"),
                        (TokenKind::Plain, r" "),
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

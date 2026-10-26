//! Hello-world snippet for `ProgrammingLanguageName::Ici`.
//!
//! Imported from leachim6/hello-world (`i/ICI.ici` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/ICI.ici>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ici => "ICI",
    versions: [Ici => "ICI"],
    extensions: [Ici => "ici"],
}

const ICI: &str = concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ici,
        extensions: &[FileExtension::Ici],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ici,
            syntax: ICI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ICI,
                    [
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };

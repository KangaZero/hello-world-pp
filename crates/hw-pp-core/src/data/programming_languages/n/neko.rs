//! Hello-world snippet for `ProgrammingLanguageName::Neko`.
//!
//! Imported from leachim6/hello-world (`n/Neko.neko` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Neko.neko>
//! - <https://www.wikidata.org/wiki/Q3337877> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    ObjectOriented, PrototypeBased, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Neko => "Neko",
    versions: [Neko => "Neko"],
    extensions: [Neko => "neko"],
}

const NEKO: &str = concat!(r#"$print(""#, output_for_hello_world!(), r#"\n");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Neko,
        extensions: &[FileExtension::Neko],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Structured, ObjectOriented, PrototypeBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Neko,
            syntax: NEKO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NEKO,
                    [
                        (TokenKind::Function, r"$print"),
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

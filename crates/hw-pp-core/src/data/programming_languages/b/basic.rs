//! Hello-world snippet for `ProgrammingLanguageName::Basic`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Dartmouth_BASIC>
//! - <https://en.wikipedia.org/wiki/BASIC>
//! - <https://www.wikidata.org/wiki/Q42979> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Basic => "BASIC",
    versions: [ClassicBasic => "Classic BASIC"],
    extensions: [Bas => "bas"],
}

const CLASSIC_BASIC: &str = concat!(
    r#"10 PRINT ""#,
    output_for_hello_world!(),
    r#""
20 END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic,
        extensions: &[FileExtension::Bas],
        categories: &[Scripting, Historical],
        families: &[Basic],
        paradigms: &[Imperative, Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClassicBasic,
            syntax: CLASSIC_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLASSIC_BASIC,
                    [
                        (TokenKind::Number, "10"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "PRINT"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Number, "20"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "END"),
                    ],
                )
            },
        }],
    };

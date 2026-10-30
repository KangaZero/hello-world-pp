//! Hello-world snippet for `ProgrammingLanguageName::Basic256`.
//!
//! Imported from leachim6/hello-world (`b/BASIC 256.kbs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BASIC%20256.kbs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Basic256 => "BASIC 256",
    versions: [Basic256 => "BASIC 256"],
    extensions: [Kbs => "kbs"],
}

const BASIC256: &str = concat!(r#"Print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic256,
        extensions: &[FileExtension::Kbs],
        categories: &[Educational],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Basic256,
            syntax: BASIC256,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BASIC256,
                    [
                        (TokenKind::Keyword, r"Print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };

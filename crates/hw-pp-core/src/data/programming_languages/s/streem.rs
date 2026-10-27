//! Hello-world snippet for `ProgrammingLanguageName::Streem`.
//!
//! Imported from leachim6/hello-world (`s/Streem.strm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Streem.strm>
//! - <https://github.com/matz/streem> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Dataflow};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Streem => "Streem",
    versions: [Streem => "Streem"],
    extensions: [Strm => "strm"],
}

const STREEM: &str = concat!(r#"[""#, output_for_hello_world!(), r#""] | stdout"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Streem,
        extensions: &[FileExtension::Strm],
        categories: &[],
        families: &[],
        paradigms: &[Concurrent, Dataflow],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Streem,
            syntax: STREEM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STREEM,
                    [(
                        TokenKind::Plain,
                        concat!(r#"[""#, output_for_hello_world!(), r#""] | stdout"#),
                    )],
                )
            },
        }],
    };

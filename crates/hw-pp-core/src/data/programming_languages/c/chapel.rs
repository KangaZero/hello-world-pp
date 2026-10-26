//! Hello-world snippet for `ProgrammingLanguageName::Chapel`.
//!
//! Imported from leachim6/hello-world (`c/Chapel.chpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Chapel.chpl>
//! - <https://www.wikidata.org/wiki/Q4036051> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Concurrent, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Chapel => "Chapel",
    versions: [Chapel => "Chapel"],
    extensions: [Chpl => "chpl"],
}

const CHAPEL: &str = concat!(r#"writeln(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chapel,
        extensions: &[FileExtension::Chpl],
        categories: &[Compiled],
        families: &[C],
        paradigms: &[ObjectOriented, Concurrent, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chapel,
            syntax: CHAPEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHAPEL,
                    [
                        (TokenKind::Function, r"writeln"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };

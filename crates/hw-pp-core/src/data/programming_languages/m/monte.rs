//! Hello-world snippet for `ProgrammingLanguageName::Monte`.
//!
//! Imported from leachim6/hello-world (`m/Monte.mt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Monte.mt>
//! - <https://esolangs.org/wiki/Monte> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Monte => "Monte",
    versions: [Monte => "Monte"],
    extensions: [Mt => "mt"],
}

const MONTE: &str = concat!(r#"traceln(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Monte,
        extensions: &[FileExtension::Mt],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Monte,
            syntax: MONTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MONTE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"traceln(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

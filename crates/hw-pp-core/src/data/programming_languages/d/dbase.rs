//! Hello-world snippet for `ProgrammingLanguageName::Dbase`.
//!
//! Imported from leachim6/hello-world (`d/dBase.dbf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/dBase.dbf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::XBase, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dbase => "dBase",
    versions: [Dbase => "dBase"],
    extensions: [Dbf => "dbf"],
}

const DBASE: &str = concat!(r#"? ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dbase,
        extensions: &[FileExtension::Dbf],
        categories: &[Historical],
        families: &[XBase],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dbase,
            syntax: DBASE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DBASE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Dbase`.
//!
//! Imported from leachim6/hello-world (`d/dBase.dbf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dbase => "dBase",
    versions: [Dbase => "dBase"],
    extensions: [Dbf => "dbf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dbase,
        extensions: &[FileExtension::Dbf],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dbase,
            syntax: concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

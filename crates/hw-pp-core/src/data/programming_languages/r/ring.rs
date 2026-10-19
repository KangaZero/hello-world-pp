//! Hello-world snippet for `ProgrammingLanguageName::Ring`.
//!
//! Imported from leachim6/hello-world (`r/Ring.ring` at `a152253`),
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
    language: Ring => "Ring",
    versions: [Ring => "Ring"],
    extensions: [Ring => "ring"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ring,
        extensions: &[FileExtension::Ring],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ring,
            syntax: concat!(r#"see ""#, output_for_hello_world!(), r#"" "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"see ""#, output_for_hello_world!(), r#"" "#),
            )],
        }],
    };

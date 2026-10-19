//! Hello-world snippet for `ProgrammingLanguageName::Rivescript`.
//!
//! Imported from leachim6/hello-world (`r/RiveScript.rive` at `a152253`),
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
    language: Rivescript => "RiveScript",
    versions: [Rivescript => "RiveScript"],
    extensions: [Rive => "rive"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rivescript,
        extensions: &[FileExtension::Rive],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rivescript,
            syntax: concat!(
                r"+ *
- ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"+ *
- ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };

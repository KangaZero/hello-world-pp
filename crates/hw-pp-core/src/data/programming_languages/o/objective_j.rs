//! Hello-world snippet for `ProgrammingLanguageName::ObjectiveJ`.
//!
//! Imported from leachim6/hello-world (`o/Objective J.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ObjectiveJ => "Objective J",
    versions: [ObjectiveJ => "Objective J"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveJ,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectiveJ,
            syntax: concat!(r#"document.write(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"document.write(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

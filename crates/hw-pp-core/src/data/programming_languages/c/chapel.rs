//! Hello-world snippet for `ProgrammingLanguageName::Chapel`.
//!
//! Imported from leachim6/hello-world (`c/Chapel.chpl` at `a152253`),
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
    language: Chapel => "Chapel",
    versions: [Chapel => "Chapel"],
    extensions: [Chpl => "chpl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chapel,
        extensions: &[FileExtension::Chpl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chapel,
            syntax: concat!(r#"writeln(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"writeln(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Fetlang`.
//!
//! Imported from leachim6/hello-world (`f/Fetlang.fet` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fetlang.fet>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fetlang => "Fetlang",
    versions: [Fetlang => "Fetlang"],
    extensions: [Fet => "fet"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fetlang,
        extensions: &[FileExtension::Fet],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fetlang,
            syntax: concat!(r#"Make slave scream ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Make slave scream ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

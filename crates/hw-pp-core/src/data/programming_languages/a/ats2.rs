//! Hello-world snippet for `ProgrammingLanguageName::Ats2`.
//!
//! Imported from leachim6/hello-world (`a/ATS2.dats` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ATS2.dats>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ats2 => "ATS2",
    versions: [Ats2 => "ATS2"],
    extensions: [Dats => "dats"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ats2,
        extensions: &[FileExtension::Dats],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ats2,
            syntax: concat!(
                r#"implement main0 () = println! ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"implement main0 () = println! ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

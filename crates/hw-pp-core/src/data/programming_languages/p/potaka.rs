//! Hello-world snippet for `ProgrammingLanguageName::Potaka`.
//!
//! Imported from leachim6/hello-world (`p/Potaka.ptk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Potaka.ptk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Potaka => "Potaka",
    versions: [Potaka => "Potaka"],
    extensions: [Ptk => "ptk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Potaka,
        extensions: &[FileExtension::Ptk],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Potaka,
            syntax: concat!(r#"দেখাও(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"দেখাও(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

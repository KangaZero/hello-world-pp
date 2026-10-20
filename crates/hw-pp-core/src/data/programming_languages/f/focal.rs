//! Hello-world snippet for `ProgrammingLanguageName::Focal`.
//!
//! Imported from leachim6/hello-world (`f/Focal.fc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Focal.fc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Joss, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Focal => "Focal",
    versions: [Focal => "Focal"],
    extensions: [Fc => "fc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Focal,
        extensions: &[FileExtension::Fc],
        categories: &[Historical],
        families: &[Joss],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Focal,
            syntax: concat!(r#"0.1.0.1 TYPE ""#, output_for_hello_world!(), r#"" , !"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"0.1.0.1 TYPE ""#, output_for_hello_world!(), r#"" , !"#),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Lingo`.
//!
//! Imported from leachim6/hello-world (`l/Lingo.lg4` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lingo.lg4>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lingo => "Lingo",
    versions: [Lingo => "Lingo"],
    extensions: [Lg4 => "lg4"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lingo,
        extensions: &[FileExtension::Lg4],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lingo,
            syntax: concat!(
                r#"on startmovie
  alert ""#,
                output_for_hello_world!(),
                r#""
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"on startmovie
  alert ""#,
                    output_for_hello_world!(),
                    r#""
end"#
                ),
            )],
        }],
    };

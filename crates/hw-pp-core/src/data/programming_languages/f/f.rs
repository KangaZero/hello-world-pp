//! Hello-world snippet for `ProgrammingLanguageName::F`.
//!
//! Imported from leachim6/hello-world (`f/F.f95` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/F.f95>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: F => "F",
    versions: [F => "F"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::F,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::F,
            syntax: concat!(
                r#"program hello
   print *, ""#,
                output_for_hello_world!(),
                r#""
end program hello"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"program hello
   print *, ""#,
                    output_for_hello_world!(),
                    r#""
end program hello"#
                ),
            )],
        }],
    };

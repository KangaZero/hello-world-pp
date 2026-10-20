//! Hello-world snippet for `ProgrammingLanguageName::Zeno`.
//!
//! Imported from leachim6/hello-world (PR #1540 `z/Zeno.zno`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1540>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zeno => "Zeno",
    versions: [Zeno => "Zeno"],
    extensions: [Zno => "zno"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zeno,
        extensions: &[FileExtension::Zno],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zeno,
            syntax: concat!(
                r#"program print_hello
  put ""#,
                output_for_hello_world!(),
                r#""
end program"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"program print_hello
  put ""#,
                    output_for_hello_world!(),
                    r#""
end program"#
                ),
            )],
        }],
    };

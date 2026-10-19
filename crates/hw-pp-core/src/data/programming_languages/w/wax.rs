//! Hello-world snippet for `ProgrammingLanguageName::Wax`.
//!
//! Imported from leachim6/hello-world (`w/Wax.wax` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Wax.wax>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wax => "Wax",
    versions: [Wax => "Wax"],
    extensions: [Wax => "wax"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wax,
        extensions: &[FileExtension::Wax],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wax,
            syntax: concat!(
                r#"(func main (result int)
  (print ""#,
                output_for_hello_world!(),
                r#"")
  (return 0)
)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(func main (result int)
  (print ""#,
                    output_for_hello_world!(),
                    r#"")
  (return 0)
)"#
                ),
            )],
        }],
    };

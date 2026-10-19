//! Hello-world snippet for `ProgrammingLanguageName::Vale`.
//!
//! Imported from leachim6/hello-world (`v/Vale.vale` at `a152253`),
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
    language: Vale => "Vale",
    versions: [Vale => "Vale"],
    extensions: [Vale => "vale"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vale,
        extensions: &[FileExtension::Vale],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vale,
            syntax: concat!(
                r#"import stdlib.*;

exported func main() {
  println(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import stdlib.*;

exported func main() {
  println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

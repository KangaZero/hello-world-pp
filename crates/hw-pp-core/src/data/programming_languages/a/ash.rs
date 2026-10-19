//! Hello-world snippet for `ProgrammingLanguageName::Ash`.
//!
//! Imported from leachim6/hello-world (`a/Ash.ash` at `a152253`),
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
    language: Ash => "Ash",
    versions: [Ash => "Ash"],
    extensions: [Ash => "ash"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ash,
        extensions: &[FileExtension::Ash],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ash,
            syntax: concat!(
                r#"public class Ash {
  public static func main(args : [String]) {
    println(""#,
                output_for_hello_world!(),
                r#"")
  }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"public class Ash {
  public static func main(args : [String]) {
    println(""#,
                    output_for_hello_world!(),
                    r#"")
  }
}"#
                ),
            )],
        }],
    };

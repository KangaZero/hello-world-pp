//! Hello-world snippet for `ProgrammingLanguageName::Ash`.
//!
//! Imported from leachim6/hello-world (`a/Ash.ash` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Ash.ash>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
        families: &[],
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

//! Hello-world snippet for `ProgrammingLanguageName::Zombie`.
//!
//! Imported from leachim6/hello-world (`z/Zombie.zombie` at `a152253`),
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
    language: Zombie => "Zombie",
    versions: [Zombie => "Zombie"],
    extensions: [Zombie => "zombie"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zombie,
        extensions: &[FileExtension::Zombie],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zombie,
            syntax: concat!(
                r#"HelloWorld is a zombie
summon
  task SayHello
      say ""#,
                output_for_hello_world!(),
                r#""
  animate
animate"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"HelloWorld is a zombie
summon
  task SayHello
      say ""#,
                    output_for_hello_world!(),
                    r#""
  animate
animate"#
                ),
            )],
        }],
    };

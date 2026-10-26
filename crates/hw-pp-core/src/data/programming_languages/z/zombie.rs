//! Hello-world snippet for `ProgrammingLanguageName::Zombie`.
//!
//! Imported from leachim6/hello-world (`z/Zombie.zombie` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zombie.zombie>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zombie => "Zombie",
    versions: [Zombie => "Zombie"],
    extensions: [Zombie => "zombie"],
}

const ZOMBIE: &str = concat!(
    r#"HelloWorld is a zombie
summon
  task SayHello
      say ""#,
    output_for_hello_world!(),
    r#""
  animate
animate"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zombie,
        extensions: &[FileExtension::Zombie],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zombie,
            syntax: ZOMBIE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZOMBIE,
                    [(
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
                )
            },
        }],
    };

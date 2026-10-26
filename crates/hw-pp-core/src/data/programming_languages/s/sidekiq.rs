//! Hello-world snippet for `ProgrammingLanguageName::Sidekiq`.
//!
//! Imported from leachim6/hello-world (`s/Sidekiq.sidekiq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sidekiq.sidekiq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ruby, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sidekiq => "Sidekiq",
    versions: [Sidekiq => "Sidekiq"],
    extensions: [Sidekiq => "sidekiq"],
}

const SIDEKIQ: &str = concat!(
    r#"def self.❨╯°□°❩╯︵┻━┻
    puts ""#,
    output_for_hello_world!(),
    r#""
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sidekiq,
        extensions: &[FileExtension::Sidekiq],
        categories: &[Framework],
        families: &[Ruby],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sidekiq,
            syntax: SIDEKIQ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SIDEKIQ,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"def self.❨╯°□°❩╯︵┻━┻
    puts ""#,
                            output_for_hello_world!(),
                            r#""
end"#
                        ),
                    )],
                )
            },
        }],
    };

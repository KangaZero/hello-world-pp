//! Hello-world snippet for `ProgrammingLanguageName::Sidekiq`.
//!
//! Imported from leachim6/hello-world (`s/Sidekiq.sidekiq` at `a152253`),
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
    language: Sidekiq => "Sidekiq",
    versions: [Sidekiq => "Sidekiq"],
    extensions: [Sidekiq => "sidekiq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sidekiq,
        extensions: &[FileExtension::Sidekiq],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sidekiq,
            syntax: concat!(
                r#"def self.❨╯°□°❩╯︵┻━┻
    puts ""#,
                output_for_hello_world!(),
                r#""
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"def self.❨╯°□°❩╯︵┻━┻
    puts ""#,
                    output_for_hello_world!(),
                    r#""
end"#
                ),
            )],
        }],
    };

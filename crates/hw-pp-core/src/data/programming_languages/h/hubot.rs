//! Hello-world snippet for `ProgrammingLanguageName::Hubot`.
//!
//! Imported from leachim6/hello-world (`h/Hubot.coffee` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hubot.coffee>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Hubot => "Hubot",
    versions: [Hubot => "Hubot"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hubot,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hubot,
            syntax: concat!(
                r#"module.exports = (robot) ->
  robot.respond /hello/i, (msg) ->
    msg.send ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module.exports = (robot) ->
  robot.respond /hello/i, (msg) ->
    msg.send ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

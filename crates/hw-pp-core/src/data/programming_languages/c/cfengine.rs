//! Hello-world snippet for `ProgrammingLanguageName::Cfengine`.
//!
//! Imported from leachim6/hello-world (`c/CFEngine.cf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CFEngine.cf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cfengine => "CFEngine",
    versions: [Cfengine => "CFEngine"],
    extensions: [Cf => "cf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cfengine,
        extensions: &[FileExtension::Cf],
        categories: &[Configuration],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cfengine,
            syntax: concat!(
                r#"body common control
{
  bundlesequence => { "run" };
}

bundle agent run
{
  reports:
    cfengine::
      ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"body common control
{
  bundlesequence => { "run" };
}

bundle agent run
{
  reports:
    cfengine::
      ""#,
                    output_for_hello_world!(),
                    r#"";
}"#
                ),
            )],
        }],
    };

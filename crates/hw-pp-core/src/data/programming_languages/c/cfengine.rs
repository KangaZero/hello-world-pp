//! Hello-world snippet for `ProgrammingLanguageName::Cfengine`.
//!
//! Imported from leachim6/hello-world (`c/CFEngine.cf` at `a152253`),
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
    language: Cfengine => "CFEngine",
    versions: [Cfengine => "CFEngine"],
    extensions: [Cf => "cf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cfengine,
        extensions: &[FileExtension::Cf],
        categories: &[],
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

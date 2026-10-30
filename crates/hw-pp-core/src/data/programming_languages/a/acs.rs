//! Hello-world snippet for `ProgrammingLanguageName::Acs`.
//!
//! Imported from leachim6/hello-world (`a/ACS.acs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ACS.acs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Acs => "ACS",
    versions: [Acs => "ACS"],
    extensions: [Acs => "acs"],
}

const ACS: &str = concat!(
    r#"#include "zcommon.acs"

Script 1 OPEN
{
  Print(s: ""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acs,
        extensions: &[FileExtension::Acs],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acs,
            syntax: ACS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"#include "zcommon.acs"

Script 1 OPEN
{
  Print(s: ""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };

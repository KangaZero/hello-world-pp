//! Hello-world snippet for `ProgrammingLanguageName::Vms`.
//!
//! Imported from leachim6/hello-world (`v/VMS.vms` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VMS.vms>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vms => "VMS",
    versions: [Vms => "VMS"],
    extensions: [Vms => "vms"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vms,
        extensions: &[FileExtension::Vms],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vms,
            syntax: concat!(
                r#"$top:
$write sys$output ""#,
                output_for_hello_world!(),
                r#""
$wait 00:00:10
$goto top"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"$top:
$write sys$output ""#,
                    output_for_hello_world!(),
                    r#""
$wait 00:00:10
$goto top"#
                ),
            )],
        }],
    };

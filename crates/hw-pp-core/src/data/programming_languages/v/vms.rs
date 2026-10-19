//! Hello-world snippet for `ProgrammingLanguageName::Vms`.
//!
//! Imported from leachim6/hello-world (`v/VMS.vms` at `a152253`),
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
    language: Vms => "VMS",
    versions: [Vms => "VMS"],
    extensions: [Vms => "vms"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vms,
        extensions: &[FileExtension::Vms],
        categories: &[],
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

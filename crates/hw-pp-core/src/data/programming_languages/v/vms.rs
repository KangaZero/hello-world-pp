//! Hello-world snippet for `ProgrammingLanguageName::Vms`.
//!
//! Imported from leachim6/hello-world (`v/VMS.vms` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VMS.vms>
//! - <https://en.wikipedia.org/wiki/DIGITAL_Command_Language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vms => "VMS",
    versions: [Vms => "VMS"],
    extensions: [Vms => "vms"],
}

const VMS: &str = concat!(
    r#"$top:
$write sys$output ""#,
    output_for_hello_world!(),
    r#""
$wait 00:00:10
$goto top"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vms,
        extensions: &[FileExtension::Vms],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vms,
            syntax: VMS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VMS,
                    [(
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
                )
            },
        }],
    };

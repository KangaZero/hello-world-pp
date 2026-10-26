//! Hello-world snippet for `ProgrammingLanguageName::VmsDcl`.
//!
//! Imported from the Hello World Collection (`files/VMS-DCL` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VMS-DCL>
//! - <http://helloworldcollection.de/#VMS-DCL>
//! - <https://en.wikipedia.org/wiki/DIGITAL_Command_Language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VmsDcl => "VMS-DCL",
    versions: [VmsDcl => "VMS-DCL"],
    extensions: [],
}

const VMS_DCL: &str = concat!(
    r"$ ! ",
    output_for_hello_world!(),
    r#" in Digital Command Language for the VMS operating system

$ WRITE SYS$OUTPUT ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VmsDcl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VmsDcl,
            syntax: VMS_DCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VMS_DCL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"$ ! ",
                            output_for_hello_world!(),
                            r#" in Digital Command Language for the VMS operating system

$ WRITE SYS$OUTPUT ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

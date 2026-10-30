//! Hello-world snippet for `ProgrammingLanguageName::AcpiSourceLanguage`.
//!
//! Imported from the Hello World Collection (`files/ACPI&nbsp;Source&nbsp;Language.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ACPI%26nbsp%3BSource%26nbsp%3BLanguage.htm>
//! - <http://helloworldcollection.de/#ACPI%C2%A0Source%C2%A0Language>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AcpiSourceLanguage => "ACPI Source Language",
    versions: [AcpiSourceLanguage => "ACPI Source Language"],
    extensions: [],
}

const ACPI_SOURCE_LANGUAGE: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in ACPI Source Language

Scope(\) {
	Method(_WAK) {
		Store (""#,
    output_for_hello_world!(),
    r#"", Debug)
		Return(Package(2){0x00000000,0})
	}	
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AcpiSourceLanguage,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AcpiSourceLanguage,
            syntax: ACPI_SOURCE_LANGUAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACPI_SOURCE_LANGUAGE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in ACPI Source Language

Scope(\) {
	Method(_WAK) {
		Store (""#,
                            output_for_hello_world!(),
                            r#"", Debug)
		Return(Package(2){0x00000000,0})
	}	
}"#
                        ),
                    )],
                )
            },
        }],
    };

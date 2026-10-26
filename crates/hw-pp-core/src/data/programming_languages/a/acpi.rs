//! Hello-world snippet for `ProgrammingLanguageName::Acpi`.
//!
//! Imported from leachim6/hello-world (`a/Acpi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Acpi>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Acpi => "Acpi",
    versions: [Acpi => "Acpi"],
    extensions: [],
}

const ACPI: &str = concat!(
    r#"Scope(\) {
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
        name: ProgrammingLanguageName::Acpi,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acpi,
            syntax: ACPI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACPI,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Scope(\) {
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

//! Hello-world snippet for `ProgrammingLanguageName::Hdx`.
//!
//! Imported from the Hello World Collection (`files/HDX.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/HDX.txt>
//! - <http://helloworldcollection.de/#HDX>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hdx => "HDX",
    versions: [Hdx => "HDX"],
    extensions: [],
}

const HDX: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" as bdehaldia.exe external command

proc hdx_info {} {
  set ::Titel [TRA "&"#,
    output_for_hello_world!(),
    r#""]
  set ::Menu GMA
}

proc hdx_run {} {
  tk_messageBox -type ok -message [TRA ""#,
    output_for_hello_world!(),
    r#""]
  destroy .
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hdx,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hdx,
            syntax: HDX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HDX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" as bdehaldia.exe external command

proc hdx_info {} {
  set ::Titel [TRA "&"#,
                            output_for_hello_world!(),
                            r#""]
  set ::Menu GMA
}

proc hdx_run {} {
  tk_messageBox -type ok -message [TRA ""#,
                            output_for_hello_world!(),
                            r#""]
  destroy .
}"#
                        ),
                    )],
                )
            },
        }],
    };

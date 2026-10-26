//! Hello-world snippet for `ProgrammingLanguageName::Nsis`.
//!
//! Imported from the Hello World Collection (`files/NSIS.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/NSIS.txt>
//! - <http://helloworldcollection.de/#NSIS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nsis => "NSIS",
    versions: [Nsis => "NSIS"],
    extensions: [],
}

const NSIS: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Nullsoft Software Install Script (NSIS)

Caption ""#,
    output_for_hello_world!(),
    r#""
OutFile ".\HelloWorld.exe"
SilentInstall silent

Section ""
        MessageBox MB_OK ""#,
    output_for_hello_world!(),
    r#""
SectionEnd"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nsis,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nsis,
            syntax: NSIS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NSIS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in Nullsoft Software Install Script (NSIS)

Caption ""#,
                            output_for_hello_world!(),
                            r#""
OutFile ".\HelloWorld.exe"
SilentInstall silent

Section ""
        MessageBox MB_OK ""#,
                            output_for_hello_world!(),
                            r#""
SectionEnd"#
                        ),
                    )],
                )
            },
        }],
    };

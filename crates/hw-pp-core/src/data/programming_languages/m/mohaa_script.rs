//! Hello-world snippet for `ProgrammingLanguageName::MohaaScript`.
//!
//! Imported from the Hello World Collection (`files/MoHAA-Script.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MoHAA-Script.txt>
//! - <http://helloworldcollection.de/#MoHAA-Script>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: MohaaScript => "MoHAA-Script",
    versions: [MohaaScript => "MoHAA-Script"],
    extensions: [],
}

const MOHAA_SCRIPT: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in the Medal of Honor Allied Assault scripting language

iprintln ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MohaaScript,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MohaaScript,
            syntax: MOHAA_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOHAA_SCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in the Medal of Honor Allied Assault scripting language

iprintln ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

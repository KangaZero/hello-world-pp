//! Hello-world snippet for `ProgrammingLanguageName::Kix`.
//!
//! Imported from the Hello World Collection (`files/Kix.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Kix.htm>
//! - <http://helloworldcollection.de/#Kix>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Kix => "Kix",
    versions: [Kix => "Kix"],
    extensions: [],
}

const KIX: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Kix
  
Run( "Notepad.exe" )
Sleep 1
SetFocus( "Untitled - Notepad" )
$ReturnCode = SendKeys(""#,
    output_for_hello_world!(),
    r#"")
Sleep( 2 )
$ReturnCode = SendKeys("~{F4}Y")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kix,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kix,
            syntax: KIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KIX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in Kix
  
Run( "Notepad.exe" )
Sleep 1
SetFocus( "Untitled - Notepad" )
$ReturnCode = SendKeys(""#,
                            output_for_hello_world!(),
                            r#"")
Sleep( 2 )
$ReturnCode = SendKeys("~{F4}Y")"#
                        ),
                    )],
                )
            },
        }],
    };

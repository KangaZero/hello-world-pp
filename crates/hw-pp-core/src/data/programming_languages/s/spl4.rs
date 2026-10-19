//! Hello-world snippet for `ProgrammingLanguageName::Spl4`.
//!
//! Imported from the Hello World Collection (`files/SPL4` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SPL4>
//! - <http://helloworldcollection.de/#SPL4>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Spl4 => "SPL4",
    versions: [Spl4 => "SPL4"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spl4,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spl4,
            syntax: concat!(
                r" HELLO: PROCEDURE OPTIONS(MAIN);
*
 /* ",
                output_for_hello_world!(),
                r" in SPL4 (Siemens) */
*
 DCL PRINTC ENTRY;
*
 CALL PRINTC('",
                output_for_hello_world!(),
                r"', 12);
 RETURN;
*
 END HELLO;"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r" HELLO: PROCEDURE OPTIONS(MAIN);
*
 /* ",
                    output_for_hello_world!(),
                    r" in SPL4 (Siemens) */
*
 DCL PRINTC ENTRY;
*
 CALL PRINTC('",
                    output_for_hello_world!(),
                    r"', 12);
 RETURN;
*
 END HELLO;"
                ),
            )],
        }],
    };

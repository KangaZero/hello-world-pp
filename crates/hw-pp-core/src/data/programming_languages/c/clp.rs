//! Hello-world snippet for `ProgrammingLanguageName::Clp`.
//!
//! Imported from the Hello World Collection (`files/CLP.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CLP.txt>
//! - <http://helloworldcollection.de/#CLP>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clp => "CLP",
    versions: [Clp => "CLP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clp,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clp,
            syntax: concat!(
                r"/* ",
                output_for_hello_world!(),
                r" in CLP for the IBM AS/400 */
PGM
SNDPGMMSG  MSG('",
                output_for_hello_world!(),
                r" !') MSGTYPE(*COMP)
 
ENDPGM"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r" in CLP for the IBM AS/400 */
PGM
SNDPGMMSG  MSG('",
                    output_for_hello_world!(),
                    r" !') MSGTYPE(*COMP)
 
ENDPGM"
                ),
            )],
        }],
    };

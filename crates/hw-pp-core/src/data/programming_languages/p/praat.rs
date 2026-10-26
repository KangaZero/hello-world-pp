//! Hello-world snippet for `ProgrammingLanguageName::Praat`.
//!
//! Imported from the Hello World Collection (`files/PRAAT.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PRAAT.txt>
//! - <http://helloworldcollection.de/#PRAAT>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Praat => "PRAAT",
    versions: [Praat => "PRAAT"],
    extensions: [],
}

const PRAAT: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r" in praat (www.praat.org)
echo ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Praat,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Praat,
            syntax: PRAAT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PRAAT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r" in praat (www.praat.org)
echo ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Elliott`.
//!
//! Imported from the Hello World Collection (`files/Elliott.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Elliott.txt>
//! - <http://helloworldcollection.de/#Elliott>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Elliott => "Elliott",
    versions: [Elliott => "Elliott"],
    extensions: [],
}

const ELLIOTT: &str = concat!(
    r":: ",
    output_for_hello_world!(),
    r" in Elliott Autocode
SETF PUNCH
SETR 1
1)TELEPRINTER
LINE
TITLE ",
    output_for_hello_world!(),
    r".;
STOP
START 1"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elliott,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elliott,
            syntax: ELLIOTT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELLIOTT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r":: ",
                            output_for_hello_world!(),
                            r" in Elliott Autocode
SETF PUNCH
SETR 1
1)TELEPRINTER
LINE
TITLE ",
                            output_for_hello_world!(),
                            r".;
STOP
START 1"
                        ),
                    )],
                )
            },
        }],
    };

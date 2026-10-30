//! Hello-world snippet for `ProgrammingLanguageName::Databasic`.
//!
//! Imported from the Hello World Collection (`files/Databasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Databasic.txt>
//! - <http://helloworldcollection.de/#Databasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Databasic => "Databasic",
    versions: [Databasic => "Databasic"],
    extensions: [],
}

const DATABASIC: &str = concat!(
    r"PROGRAM HELLO.B
# ",
    output_for_hello_world!(),
    r#" in Databasic
CRT "HELLOW WORLD"
END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Databasic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Databasic,
            syntax: DATABASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DATABASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"PROGRAM HELLO.B
# ",
                            output_for_hello_world!(),
                            r#" in Databasic
CRT "HELLOW WORLD"
END"#
                        ),
                    )],
                )
            },
        }],
    };

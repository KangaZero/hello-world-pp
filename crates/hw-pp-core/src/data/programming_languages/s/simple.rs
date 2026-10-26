//! Hello-world snippet for `ProgrammingLanguageName::Simple`.
//!
//! Imported from the Hello World Collection (`files/SIMPLE.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SIMPLE.txt>
//! - <http://helloworldcollection.de/#SIMPLE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Simple => "SIMPLE",
    versions: [Simple => "SIMPLE"],
    extensions: [],
}

const SIMPLE: &str = concat!(
    r"[::PROGRAM:",
    output_for_hello_world!(),
    r" program in SIMPLE
A EQL @0
MSG A
END
]
{::DATA:Data part
@0:T
",
    output_for_hello_world!(),
    r"$$M
$$@
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Simple,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Simple,
            syntax: SIMPLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SIMPLE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"[::PROGRAM:",
                            output_for_hello_world!(),
                            r" program in SIMPLE
A EQL @0
MSG A
END
]
{::DATA:Data part
@0:T
",
                            output_for_hello_world!(),
                            r"$$M
$$@
}"
                        ),
                    )],
                )
            },
        }],
    };

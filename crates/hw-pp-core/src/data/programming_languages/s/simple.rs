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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Simple => "SIMPLE",
    versions: [Simple => "SIMPLE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Simple,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Simple,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };

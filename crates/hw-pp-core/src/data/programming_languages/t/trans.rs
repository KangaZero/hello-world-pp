//! Hello-world snippet for `ProgrammingLanguageName::Trans`.
//!
//! Imported from the Hello World Collection (`files/Trans.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Trans.txt>
//! - <http://helloworldcollection.de/#Trans>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Trans => "Trans",
    versions: [Trans => "Trans"],
    extensions: [],
}

const TRANS: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Trans (Transmuter Programming Language)
import Console
Console.write(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trans,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trans,
            syntax: TRANS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TRANS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in Trans (Transmuter Programming Language)
import Console
Console.write(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Rey`.
//!
//! Imported from the Hello World Collection (`files/Rey.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Rey.htm>
//! - <http://helloworldcollection.de/#Rey>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rey => "Rey",
    versions: [Rey => "Rey"],
    extensions: [],
}

const REY: &str = concat!(
    r"//beginning ",
    output_for_hello_world!(),
    r#" in Rey:
 
korzystaj "Konsola";
 
program
{
    PiszLinia(""#,
    output_for_hello_world!(),
    r#"");
}
//end "#,
    output_for_hello_world!(),
    r" in Rey"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rey,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rey,
            syntax: REY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REY,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"//beginning ",
                            output_for_hello_world!(),
                            r#" in Rey:
 
korzystaj "Konsola";
 
program
{
    PiszLinia(""#,
                            output_for_hello_world!(),
                            r#"");
}
//end "#,
                            output_for_hello_world!(),
                            r" in Rey"
                        ),
                    )],
                )
            },
        }],
    };

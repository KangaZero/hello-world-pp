//! Hello-world snippet for `ProgrammingLanguageName::Gramophone`.
//!
//! Imported from the Hello World Collection (`files/GRAMophone.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/GRAMophone.txt>
//! - <http://helloworldcollection.de/#GRAMophone>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gramophone => "GRAMophone",
    versions: [Gramophone => "GRAMophone"],
    extensions: [],
}

const GRAMOPHONE: &str = concat!(
    r"//",
    output_for_hello_world!(),
    r#" in GRAMophone

composition ""#,
    output_for_hello_world!(),
    r#"" of "Composer"
{
 %
 player player1 {
    grammar lindenmayer
    %
    axiom->print(""#,
    output_for_hello_world!(),
    r#"");
 }

 player player2 {
    grammar chomsky
    %
       @composition->print(""#,
    output_for_hello_world!(),
    r#"");
 }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gramophone,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gramophone,
            syntax: GRAMOPHONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GRAMOPHONE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"//",
                            output_for_hello_world!(),
                            r#" in GRAMophone

composition ""#,
                            output_for_hello_world!(),
                            r#"" of "Composer"
{
 %
 player player1 {
    grammar lindenmayer
    %
    axiom->print(""#,
                            output_for_hello_world!(),
                            r#"");
 }

 player player2 {
    grammar chomsky
    %
       @composition->print(""#,
                            output_for_hello_world!(),
                            r#"");
 }
}"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Eos2`.
//!
//! Imported from the Hello World Collection (`files/EOS&nbsp;2.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/EOS%26nbsp%3B2.htm>
//! - <http://helloworldcollection.de/#EOS%C2%A02>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Eos2 => "EOS 2",
    versions: [Eos2 => "EOS 2"],
    extensions: [],
}

const EOS2: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in EOS 2

Fenster:Fenster
Text:Textfeld

Fenster.zeichne(Text)
Text.zeileHinzufügen(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eos2,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eos2,
            syntax: EOS2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EOS2,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in EOS 2

Fenster:Fenster
Text:Textfeld

Fenster.zeichne(Text)
Text.zeileHinzufügen(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };

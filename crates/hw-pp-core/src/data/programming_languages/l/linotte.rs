//! Hello-world snippet for `ProgrammingLanguageName::Linotte`.
//!
//! Imported from the Hello World Collection (`files/Linotte.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Linotte.htm>
//! - <http://helloworldcollection.de/#Linotte>
//! - <https://www.wikidata.org/wiki/Q3241663> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Linotte => "Linotte",
    versions: [Linotte => "Linotte"],
    extensions: [],
}

const LINOTTE: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Linotte

BonjourLeMonde:
   début
     affiche ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Linotte,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Linotte,
            syntax: LINOTTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LINOTTE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in Linotte

BonjourLeMonde:
   début
     affiche ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

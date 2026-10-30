//! Hello-world snippet for `ProgrammingLanguageName::Ti8x`.
//!
//! Imported from the Hello World Collection (`files/TI-8x` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TI-8x>
//! - <http://helloworldcollection.de/#TI-8x>
//! - <https://en.wikipedia.org/wiki/TI-BASIC_83> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ti8x => "TI-8x",
    versions: [Ti8x => "TI-8x"],
    extensions: [],
}

const TI8X: &str = concat!(
    output_for_hello_world!(),
    r#" for TI 8x/9x basic (tested on a TI-83)

:ClrHome
:Disp ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ti8x,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ti8x,
            syntax: TI8X,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TI8X,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r#" for TI 8x/9x basic (tested on a TI-83)

:ClrHome
:Disp ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

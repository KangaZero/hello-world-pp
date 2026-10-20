//! Hello-world snippet for `ProgrammingLanguageName::PovRay`.
//!
//! Imported from the Hello World Collection (`files/POV-Ray.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/POV-Ray.htm>
//! - <http://helloworldcollection.de/#POV-Ray>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PovRay => "POV-Ray",
    versions: [PovRay => "POV-Ray"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PovRay,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PovRay,
            syntax: concat!(
                r#"#include "skies.inc"

camera{location  look_at y}
light_source{,2}
sky_sphere{S_Cloud1}
plane{y 0 pigment{checker rgb 1,0} finish{reflection .5}}
text{ttf "timrom.ttf" ""#,
                output_for_hello_world!(),
                r#"" .3,0
  pigment {agate scale .2} translate -2.5*x
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include "skies.inc"

camera{location  look_at y}
light_source{,2}
sky_sphere{S_Cloud1}
plane{y 0 pigment{checker rgb 1,0} finish{reflection .5}}
text{ttf "timrom.ttf" ""#,
                    output_for_hello_world!(),
                    r#"" .3,0
  pigment {agate scale .2} translate -2.5*x
}"#
                ),
            )],
        }],
    };

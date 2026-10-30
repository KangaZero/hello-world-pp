//! Hello-world snippet for `ProgrammingLanguageName::Velocity`.
//!
//! Imported from the Hello World Collection (`files/Velocity.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Velocity.txt>
//! - <http://helloworldcollection.de/#Velocity>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Velocity => "Velocity",
    versions: [Velocity => "Velocity"],
    extensions: [],
}

const VELOCITY: &str = concat!(
    r"<HTML>
<!-- ",
    output_for_hello_world!(),
    r#" in Velocity -->
<BODY>
  #set( $foo = ""#,
    output_for_hello_world!(),
    r#"" )
  $foo
</BODY>
</HTML>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Velocity,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Velocity,
            syntax: VELOCITY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VELOCITY,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<HTML>
<!-- ",
                            output_for_hello_world!(),
                            r#" in Velocity -->
<BODY>
  #set( $foo = ""#,
                            output_for_hello_world!(),
                            r#"" )
  $foo
</BODY>
</HTML>"#
                        ),
                    )],
                )
            },
        }],
    };

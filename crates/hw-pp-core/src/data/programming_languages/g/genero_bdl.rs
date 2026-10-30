//! Hello-world snippet for `ProgrammingLanguageName::GeneroBdl`.
//!
//! Imported from the Hello World Collection (`files/Genero&nbsp;BDL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Genero%26nbsp%3BBDL.htm>
//! - <http://helloworldcollection.de/#Genero%C2%A0BDL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: GeneroBdl => "Genero BDL",
    versions: [GeneroBdl => "Genero BDL"],
    extensions: [],
}

const GENERO_BDL: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r#" in Genero BDL

main
   display ""#,
    output_for_hello_world!(),
    r#""
end main"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GeneroBdl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GeneroBdl,
            syntax: GENERO_BDL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GENERO_BDL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"-- ",
                            output_for_hello_world!(),
                            r#" in Genero BDL

main
   display ""#,
                            output_for_hello_world!(),
                            r#""
end main"#
                        ),
                    )],
                )
            },
        }],
    };

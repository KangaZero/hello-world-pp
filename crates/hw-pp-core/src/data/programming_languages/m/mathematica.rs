//! Hello-world snippet for `ProgrammingLanguageName::Mathematica`.
//!
//! Imported from leachim6/hello-world (`m/Mathematica.nb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mathematica.nb>
//! - <https://www.wikidata.org/wiki/Q81294> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mathematica => "Mathematica",
    versions: [Mathematica => "Mathematica"],
    extensions: [Nb => "nb"],
}

const MATHEMATICA: &str = concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mathematica,
        extensions: &[FileExtension::Nb],
        categories: &[Scientific],
        families: &[],
        paradigms: &[Functional, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mathematica,
            syntax: MATHEMATICA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATHEMATICA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::CasioBasic`.
//!
//! Imported from leachim6/hello-world (`c/Casio BASIC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Casio%20BASIC>
//! - <https://en.wikipedia.org/wiki/Casio_BASIC> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: CasioBasic => "Casio BASIC",
    versions: [CasioBasic => "Casio BASIC"],
    extensions: [],
}

const CASIO_BASIC: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CasioBasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CasioBasic,
            syntax: CASIO_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CASIO_BASIC,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

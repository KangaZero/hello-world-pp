//! Hello-world snippet for `ProgrammingLanguageName::Q`.
//!
//! Imported from leachim6/hello-world (`q/Q.q` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Q.q>
//! - <https://www.wikidata.org/wiki/Q7265638> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Apl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Q => "Q",
    versions: [Q => "Q"],
    extensions: [Q => "q"],
}

const Q: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Q,
        extensions: &[FileExtension::Q],
        categories: &[Scripting],
        families: &[Apl],
        paradigms: &[Functional, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Q,
            syntax: Q,
            tokenized_syntax: &const {
                checked_token_lengths(
                    Q,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

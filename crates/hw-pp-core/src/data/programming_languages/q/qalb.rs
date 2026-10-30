//! Hello-world snippet for `ProgrammingLanguageName::Qalb`.
//!
//! Imported from leachim6/hello-world (`q/Qalb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Qalb>
//! - <https://www.wikidata.org/wiki/Q7265978> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Qalb => "Qalb",
    versions: [Qalb => "Qalb"],
    extensions: [],
}

const QALB: &str = concat!(r#"(قول ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qalb,
        extensions: &[],
        categories: &[],
        families: &[Lisp],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qalb,
            syntax: QALB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QALB,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(قول ""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

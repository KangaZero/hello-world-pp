//! Hello-world snippet for `ProgrammingLanguageName::Topshell`.
//!
//! Imported from leachim6/hello-world (`t/TopShell` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TopShell>
//! - <https://github.com/topshell-language/topshell> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Reactive};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Topshell => "TopShell",
    versions: [Topshell => "TopShell"],
    extensions: [],
}

const TOPSHELL: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Topshell,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Functional, Reactive],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Topshell,
            syntax: TOPSHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TOPSHELL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::PikeletBang`.
//!
//! Imported from leachim6/hello-world (`p/Pikelet!.pi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pikelet%21.pi>
//! - <https://github.com/pikelet-lang/pikelet> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PikeletBang => "Pikelet!",
    versions: [PikeletBang => "Pikelet!"],
    extensions: [],
}

const PIKELET_BANG: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PikeletBang,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PikeletBang,
            syntax: PIKELET_BANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PIKELET_BANG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Hack`.
//!
//! Imported from leachim6/hello-world (`h/Hack.hh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hack.hh>
//! - <https://en.wikipedia.org/wiki/Hack_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hack => "Hack",
    versions: [Hack => "Hack"],
    extensions: [Hh => "hh"],
}

const HACK: &str = concat!(
    r#"<?hh 
echo ""#,
    output_for_hello_world!(),
    r#"";"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hack,
        extensions: &[FileExtension::Hh],
        categories: &[],
        families: &[],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hack,
            syntax: HACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HACK,
                    [
                        (TokenKind::Punctuation, r"<?"),
                        (
                            TokenKind::Plain,
                            r"hh 
",
                        ),
                        (TokenKind::Function, r"echo"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };

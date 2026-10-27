//! Hello-world snippet for `ProgrammingLanguageName::Retro`.
//!
//! Imported from leachim6/hello-world (`r/Retro.retro` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Retro.retro>
//! - <https://en.wikipedia.org/wiki/Forth_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concatenative, Procedural, Reflective, StackBased};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Retro => "Retro",
    versions: [Retro => "Retro"],
    extensions: [Retro => "retro"],
}

const RETRO: &str = concat!(r#"""#, output_for_hello_world!(), r#"" puts"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Retro,
        extensions: &[FileExtension::Retro],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, Reflective, StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Retro,
            syntax: RETRO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RETRO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" puts"#),
                    )],
                )
            },
        }],
    };

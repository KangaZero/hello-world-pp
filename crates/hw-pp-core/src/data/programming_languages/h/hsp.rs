//! Hello-world snippet for `ProgrammingLanguageName::Hsp`.
//!
//! Imported from leachim6/hello-world (`h/HSP.hsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HSP.hsp>
//! - <https://en.wikipedia.org/wiki/Hot_Soup_Processor> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hsp => "HSP",
    versions: [Hsp => "HSP"],
    extensions: [Hsp => "hsp"],
}

const HSP: &str = concat!(
    r#"mes ""#,
    output_for_hello_world!(),
    r#""
stop"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hsp,
        extensions: &[FileExtension::Hsp],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hsp,
            syntax: HSP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HSP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"mes ""#,
                            output_for_hello_world!(),
                            r#""
stop"#
                        ),
                    )],
                )
            },
        }],
    };

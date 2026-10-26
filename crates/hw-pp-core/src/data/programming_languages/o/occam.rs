//! Hello-world snippet for `ProgrammingLanguageName::Occam`.
//!
//! Imported from leachim6/hello-world (`o/occam.occam` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/occam.occam>
//! - <https://www.wikidata.org/wiki/Q838062> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Imperative, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Occam => "occam",
    versions: [Occam => "occam"],
    extensions: [Occam => "occam"],
}

const OCCAM: &str = concat!(
    r#"PROGRAM Hello
#USE ioconv

SEQ
  write.full.string(screen,""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Occam,
        extensions: &[FileExtension::Occam],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Occam,
            syntax: OCCAM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OCCAM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"PROGRAM Hello
#USE ioconv

SEQ
  write.full.string(screen,""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };

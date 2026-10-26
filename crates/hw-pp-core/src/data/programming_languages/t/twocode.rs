//! Hello-world snippet for `ProgrammingLanguageName::Twocode`.
//!
//! Imported from leachim6/hello-world (`t/Twocode.dcode` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Twocode.dcode>
//! - <https://esolangs.org/wiki/Twocode> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Twocode => "Twocode",
    versions: [Twocode => "Twocode"],
    extensions: [Dcode => "dcode"],
}

const TWOCODE: &str = concat!(
    r#"{ i~0;                 /={i~i+1}============\
  *i~""#,
    output_for_hello_world!(),
    r#"\n" }=>={putc(*i)}={!*i}=S=/
                                          #"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Twocode,
        extensions: &[FileExtension::Dcode],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Twocode,
            syntax: TWOCODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TWOCODE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"{ i~0;                 /={i~i+1}============\
  *i~""#,
                            output_for_hello_world!(),
                            r#"\n" }=>={putc(*i)}={!*i}=S=/
                                          #"#
                        ),
                    )],
                )
            },
        }],
    };

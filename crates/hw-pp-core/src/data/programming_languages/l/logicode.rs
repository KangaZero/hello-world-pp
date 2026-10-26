//! Hello-world snippet for `ProgrammingLanguageName::Logicode`.
//!
//! Imported from leachim6/hello-world (`l/Logicode.lgc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Logicode.lgc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Logicode => "Logicode",
    versions: [Logicode => "Logicode"],
    extensions: [Lgc => "lgc"],
}

const LOGICODE: &str = r"out @1001000+@1100101+@1101100+@1101100+@1101111+@100000+@1010111+@1101111+@1110010+@1101100+@1100100";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logicode,
        extensions: &[FileExtension::Lgc],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logicode,
            syntax: LOGICODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOGICODE,
                    [(
                        TokenKind::Plain,
                        r"out @1001000+@1100101+@1101100+@1101100+@1101111+@100000+@1010111+@1101111+@1110010+@1101100+@1100100",
                    )],
                )
            },
        }],
    };

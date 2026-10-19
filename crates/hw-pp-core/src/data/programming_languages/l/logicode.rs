//! Hello-world snippet for `ProgrammingLanguageName::Logicode`.
//!
//! Imported from leachim6/hello-world (`l/Logicode.lgc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Logicode => "Logicode",
    versions: [Logicode => "Logicode"],
    extensions: [Lgc => "lgc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logicode,
        extensions: &[FileExtension::Lgc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logicode,
            syntax: r"out @1001000+@1100101+@1101100+@1101100+@1101111+@100000+@1010111+@1101111+@1110010+@1101100+@1100100",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"out @1001000+@1100101+@1101100+@1101100+@1101111+@100000+@1010111+@1101111+@1110010+@1101100+@1100100",
            )],
        }],
    };

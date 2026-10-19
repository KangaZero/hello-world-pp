//! Hello-world snippet for `ProgrammingLanguageName::Tis100`.
//!
//! Imported from leachim6/hello-world (PR #1853 `t/TIS-100.tis`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Tis100 => "TIS-100",
    versions: [Tis100 => "TIS-100"],
    extensions: [Tis => "tis"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tis100,
        extensions: &[FileExtension::Tis],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tis100,
            syntax: r"@0 
MOV 72 DOWN 
MOV 101 DOWN 
MOV 108 DOWN 
MOV 108 DOWN 
MOV 111 DOWN 
MOV 44 DOWN 
MOV 32 DOWN 
MOV 87 DOWN 
MOV 111 DOWN 
MOV 114 DOWN 
MOV 108 DOWN 
MOV 100 DOWN 
MOV 33 DOWN 
MOV 10 DOWN 
HCF",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"@0 
MOV 72 DOWN 
MOV 101 DOWN 
MOV 108 DOWN 
MOV 108 DOWN 
MOV 111 DOWN 
MOV 44 DOWN 
MOV 32 DOWN 
MOV 87 DOWN 
MOV 111 DOWN 
MOV 114 DOWN 
MOV 108 DOWN 
MOV 100 DOWN 
MOV 33 DOWN 
MOV 10 DOWN 
HCF",
            )],
        }],
    };

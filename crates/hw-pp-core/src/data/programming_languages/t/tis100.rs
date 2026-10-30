//! Hello-world snippet for `ProgrammingLanguageName::Tis100`.
//!
//! Imported from leachim6/hello-world (PR #1853 `t/TIS-100.tis`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1853>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Tis100 => "TIS-100",
    versions: [Tis100 => "TIS-100"],
    extensions: [Tis => "tis"],
}

const TIS100: &str = r"@0 
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
HCF";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tis100,
        extensions: &[FileExtension::Tis],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tis100,
            syntax: TIS100,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TIS100,
                    [(
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
                )
            },
        }],
    };

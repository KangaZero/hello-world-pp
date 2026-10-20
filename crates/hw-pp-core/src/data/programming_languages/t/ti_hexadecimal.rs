//! Hello-world snippet for `ProgrammingLanguageName::TiHexadecimal`.
//!
//! Imported from leachim6/hello-world (`t/TI Hexadecimal.hex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TI%20Hexadecimal.hex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: TiHexadecimal => "TI Hexadecimal",
    versions: [TiHexadecimal => "TI Hexadecimal"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TiHexadecimal,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TiHexadecimal,
            syntax: r"AsmPrgm
EF4045
111A1A
ED53D786
21A89D
EF6145
C9
48656C6C6F20776F726C64202100 ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"AsmPrgm
EF4045
111A1A
ED53D786
21A89D
EF6145
C9
48656C6C6F20776F726C64202100 ",
            )],
        }],
    };

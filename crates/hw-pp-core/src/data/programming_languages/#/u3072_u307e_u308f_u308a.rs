//! Hello-world snippet for `ProgrammingLanguageName::U3072U307eU308fU308a`.
//!
//! Imported from leachim6/hello-world (`#/ひまわり.hmw` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E3%81%B2%E3%81%BE%E3%82%8F%E3%82%8A.hmw>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: U3072U307eU308fU308a => "ひまわり",
    versions: [U3072U307eU308fU308a => "ひまわり"],
    extensions: [Hmw => "hmw"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U3072U307eU308fU308a,
        extensions: &[FileExtension::Hmw],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U3072U307eU308fU308a,
            syntax: concat!(r"「", output_for_hello_world!(), r"」と、表示。"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」と、表示。"),
            )],
        }],
    };

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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: U3072U307eU308fU308a => "ひまわり",
    versions: [U3072U307eU308fU308a => "ひまわり"],
    extensions: [Hmw => "hmw"],
}

const U3072_U307E_U308F_U308A: &str = concat!(r"「", output_for_hello_world!(), r"」と、表示。");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U3072U307eU308fU308a,
        extensions: &[FileExtension::Hmw],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U3072U307eU308fU308a,
            syntax: U3072_U307E_U308F_U308A,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U3072_U307E_U308F_U308A,
                    [(
                        TokenKind::Plain,
                        concat!(r"「", output_for_hello_world!(), r"」と、表示。"),
                    )],
                )
            },
        }],
    };

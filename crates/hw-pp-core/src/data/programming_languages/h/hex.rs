//! Hello-world snippet for `ProgrammingLanguageName::Hex`.
//!
//! Imported from leachim6/hello-world (`h/HEX.hex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HEX.hex>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Hex => "HEX",
    versions: [Hex => "HEX"],
    extensions: [Hex => "hex"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hex,
        extensions: &[FileExtension::Hex],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hex,
            syntax: concat!(
                r#"GBL;
Bug("1", ""#,
                output_for_hello_world!(),
                r#"");
Scuttle("1");
Write;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"GBL;
Bug("1", ""#,
                    output_for_hello_world!(),
                    r#"");
Scuttle("1");
Write;"#
                ),
            )],
        }],
    };

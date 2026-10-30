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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Hex => "HEX",
    versions: [Hex => "HEX"],
    extensions: [Hex => "hex"],
}

const HEX: &str = concat!(
    r#"GBL;
Bug("1", ""#,
    output_for_hello_world!(),
    r#"");
Scuttle("1");
Write;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hex,
        extensions: &[FileExtension::Hex],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hex,
            syntax: HEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HEX,
                    [(
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
                )
            },
        }],
    };

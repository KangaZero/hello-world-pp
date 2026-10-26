//! Hello-world snippet for `ProgrammingLanguageName::Idl`.
//!
//! Imported from leachim6/hello-world (`i/IDL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IDL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scientific, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Idl => "IDL",
    versions: [Idl => "IDL"],
    extensions: [],
}

const IDL: &str = concat!(
    r#"print, ""#,
    output_for_hello_world!(),
    r#""
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idl,
        extensions: &[],
        categories: &[Scientific],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idl,
            syntax: IDL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IDL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"print, ""#,
                            output_for_hello_world!(),
                            r#""
end"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Gml`.
//!
//! Imported from leachim6/hello-world (`g/GML.gml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GML.gml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gml => "GML",
    versions: [Gml => "GML"],
    extensions: [Gml => "gml"],
}

const GML: &str = concat!(r#"draw_text(1, 1, ""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gml,
        extensions: &[FileExtension::Gml],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gml,
            syntax: GML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GML,
                    [(
                        TokenKind::Plain,
                        concat!(r#"draw_text(1, 1, ""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };

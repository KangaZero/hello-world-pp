//! Hello-world snippet for `ProgrammingLanguageName::Mivascript`.
//!
//! Imported from the Hello World Collection (`files/MivaScript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MivaScript.txt>
//! - <http://helloworldcollection.de/#MivaScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mivascript => "MivaScript",
    versions: [Mivascript => "MivaScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mivascript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mivascript,
            syntax: concat!(
                r"<MvCOMMENT>",
                output_for_hello_world!(),
                r#" in Miva Script</MvCOMMENT>
<MvEVAL EXPR="{'"#,
                output_for_hello_world!(),
                r#"'}">"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<MvCOMMENT>",
                    output_for_hello_world!(),
                    r#" in Miva Script</MvCOMMENT>
<MvEVAL EXPR="{'"#,
                    output_for_hello_world!(),
                    r#"'}">"#
                ),
            )],
        }],
    };

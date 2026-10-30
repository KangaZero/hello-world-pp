//! Hello-world snippet for `ProgrammingLanguageName::Mivascript`.
//!
//! Imported from the Hello World Collection (`files/MivaScript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MivaScript.txt>
//! - <http://helloworldcollection.de/#MivaScript>
//! - <https://en.wikipedia.org/wiki/MIVA_Script> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mivascript => "MivaScript",
    versions: [Mivascript => "MivaScript"],
    extensions: [],
}

const MIVASCRIPT: &str = concat!(
    r"<MvCOMMENT>",
    output_for_hello_world!(),
    r#" in Miva Script</MvCOMMENT>
<MvEVAL EXPR="{'"#,
    output_for_hello_world!(),
    r#"'}">"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mivascript,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mivascript,
            syntax: MIVASCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIVASCRIPT,
                    [(
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
                )
            },
        }],
    };

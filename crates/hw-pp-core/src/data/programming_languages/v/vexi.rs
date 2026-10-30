//! Hello-world snippet for `ProgrammingLanguageName::Vexi`.
//!
//! Imported from the Hello World Collection (`files/Vexi.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Vexi.htm>
//! - <http://helloworldcollection.de/#Vexi>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Vexi => "Vexi",
    versions: [Vexi => "Vexi"],
    extensions: [],
}

const VEXI: &str = concat!(
    r"<!-- ",
    output_for_hello_world!(),
    r#" in Vexi. -->

<vexi xmlns:ui="vexi://ui">
    <ui:box framewidth="200" frameheight="100">
        <ui:box text=""#,
    output_for_hello_world!(),
    r#"" />
        vexi.ui.frame = thisbox;
    </ui:box>
</vexi>
/<"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vexi,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vexi,
            syntax: VEXI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VEXI,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<!-- ",
                            output_for_hello_world!(),
                            r#" in Vexi. -->

<vexi xmlns:ui="vexi://ui">
    <ui:box framewidth="200" frameheight="100">
        <ui:box text=""#,
                            output_for_hello_world!(),
                            r#"" />
        vexi.ui.frame = thisbox;
    </ui:box>
</vexi>
/<"#
                        ),
                    )],
                )
            },
        }],
    };

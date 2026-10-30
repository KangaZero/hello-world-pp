//! Hello-world snippet for `ProgrammingLanguageName::RebolView`.
//!
//! Imported from the Hello World Collection (`files/Rebol-view.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Rebol-view.txt>
//! - <http://helloworldcollection.de/#Rebol-view>
//! - <https://en.wikipedia.org/wiki/Rebol> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, PrototypeBased,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: RebolView => "Rebol-view",
    versions: [RebolView => "Rebol-view"],
    extensions: [],
}

const REBOL_VIEW: &str = concat!(
    output_for_hello_world!(),
    r#" in Rebol-view.

rebol[]
view layout[
text ""#,
    output_for_hello_world!(),
    r#""
]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RebolView,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, PrototypeBased, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RebolView,
            syntax: REBOL_VIEW,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REBOL_VIEW,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r#" in Rebol-view.

rebol[]
view layout[
text ""#,
                            output_for_hello_world!(),
                            r#""
]"#
                        ),
                    )],
                )
            },
        }],
    };

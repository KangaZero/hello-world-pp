//! Hello-world snippet for `ProgrammingLanguageName::PineScript`.
//!
//! Imported from leachim6/hello-world (PR #1822 `p/Pinescript.pine`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1822>
//! - <https://www.tradingview.com/pine-script-docs/language/execution-model/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::EventDriven;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: PineScript => "Pine Script",
    versions: [PineScript => "Pine Script"],
    extensions: [Pine => "pine"],
}

const PINE_SCRIPT: &str = concat!(
    r#"//@version=5
indicator(""#,
    output_for_hello_world!(),
    r#"")
label.new(bar_index, high, ""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PineScript,
        extensions: &[FileExtension::Pine],
        categories: &[Scripting],
        families: &[],
        paradigms: &[EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PineScript,
            syntax: PINE_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PINE_SCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"//@version=5
indicator(""#,
                            output_for_hello_world!(),
                            r#"")
label.new(bar_index, high, ""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Cybol`.
//!
//! Imported from the Hello World Collection (`files/CYBOL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CYBOL.txt>
//! - <http://helloworldcollection.de/#CYBOL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cybol => "CYBOL",
    versions: [Cybol => "CYBOL"],
    extensions: [],
}

const CYBOL: &str = concat!(
    r"<!-- ",
    output_for_hello_world!(),
    r#" in Cybernetics Oriented Language (CYBOL) -->
<model>
    <part name="send_message" channel="inline" abstraction="operation" model="send">
        <property name="channel" channel="inline" abstraction="character" model="shell"/>
        <property name="message" channel="inline" abstraction="character" model=""#,
    output_for_hello_world!(),
    r#""/>
    </part>
    <part name="exit_application" channel="inline" abstraction="operation" model="exit"/>
</model>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cybol,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cybol,
            syntax: CYBOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CYBOL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<!-- ",
                            output_for_hello_world!(),
                            r#" in Cybernetics Oriented Language (CYBOL) -->
<model>
    <part name="send_message" channel="inline" abstraction="operation" model="send">
        <property name="channel" channel="inline" abstraction="character" model="shell"/>
        <property name="message" channel="inline" abstraction="character" model=""#,
                            output_for_hello_world!(),
                            r#""/>
    </part>
    <part name="exit_application" channel="inline" abstraction="operation" model="exit"/>
</model>"#
                        ),
                    )],
                )
            },
        }],
    };

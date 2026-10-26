//! Hello-world snippet for `ProgrammingLanguageName::Codscript`.
//!
//! Imported from the Hello World Collection (`files/CoDScript.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CoDScript.htm>
//! - <http://helloworldcollection.de/#CoDScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Codscript => "CoDScript",
    versions: [Codscript => "CoDScript"],
    extensions: [],
}

const CODSCRIPT: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in CoDScript

main(){
     iPrintLnBold(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Codscript,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Codscript,
            syntax: CODSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CODSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in CoDScript

main(){
     iPrintLnBold(""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::OU02f8Xml`.
//!
//! Imported from leachim6/hello-world (`o/o˸XML` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: OU02f8Xml => "o˸XML",
    versions: [OU02f8Xml => "o˸XML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OU02f8Xml,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OU02f8Xml,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<text><![CDATA["#,
                output_for_hello_world!(),
                r"]]></text>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<text><![CDATA["#,
                    output_for_hello_world!(),
                    r"]]></text>"
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Xul`.
//!
//! Imported from leachim6/hello-world (`x/XUL.xul` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xul => "XUL",
    versions: [Xul => "XUL"],
    extensions: [Xul => "xul"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xul,
        extensions: &[FileExtension::Xul],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xul,
            syntax: concat!(
                r#"<?xml version="1.0"?>
<window xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">
    <label value=""#,
                output_for_hello_world!(),
                r#""/>
</window>"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<?xml version="1.0"?>
<window xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">
    <label value=""#,
                    output_for_hello_world!(),
                    r#""/>
</window>"#
                ),
            )],
        }],
    };

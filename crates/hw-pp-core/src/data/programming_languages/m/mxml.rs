//! Hello-world snippet for `ProgrammingLanguageName::Mxml`.
//!
//! Imported from leachim6/hello-world (`m/MXML.mxml` at `a152253`),
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
    language: Mxml => "MXML",
    versions: [Mxml => "MXML"],
    extensions: [Mxml => "mxml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mxml,
        extensions: &[FileExtension::Mxml],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mxml,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<mx:Application xmlns:mx="http://www.adobe.com/2006/mxml">
<mx:Label text=""#,
                output_for_hello_world!(),
                r#""/>
</mx:Application>"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<?xml version="1.0" encoding="utf-8"?>
<mx:Application xmlns:mx="http://www.adobe.com/2006/mxml">
<mx:Label text=""#,
                    output_for_hello_world!(),
                    r#""/>
</mx:Application>"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Zinc`.
//!
//! Imported from leachim6/hello-world (`z/Zinc.zn` at `a152253`),
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
    language: Zinc => "Zinc",
    versions: [Zinc => "Zinc"],
    extensions: [Zn => "zn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zinc,
        extensions: &[FileExtension::Zn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zinc,
            syntax: concat!(
                r#"//! zinc
library HelloWorld {
    private function onInit() {
        DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
                output_for_hello_world!(),
                r#"");
    }
}
//! endzinc"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"//! zinc
library HelloWorld {
    private function onInit() {
        DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
                    output_for_hello_world!(),
                    r#"");
    }
}
//! endzinc"#
                ),
            )],
        }],
    };

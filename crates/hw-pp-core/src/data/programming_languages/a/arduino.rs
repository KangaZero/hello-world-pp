//! Hello-world snippet for `ProgrammingLanguageName::Arduino`.
//!
//! Imported from leachim6/hello-world (`a/Arduino.ino` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Arduino.ino>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Arduino => "Arduino",
    versions: [Arduino => "Arduino"],
    extensions: [Ino => "ino"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arduino,
        extensions: &[FileExtension::Ino],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arduino,
            syntax: concat!(
                r#"void setup() {
  Serial.begin(9600);
  Serial.println(""#,
                output_for_hello_world!(),
                r#"");
}

void loop() {

}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"void setup() {
  Serial.begin(9600);
  Serial.println(""#,
                    output_for_hello_world!(),
                    r#"");
}

void loop() {

}"#
                ),
            )],
        }],
    };

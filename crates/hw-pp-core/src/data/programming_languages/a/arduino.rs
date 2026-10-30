//! Hello-world snippet for `ProgrammingLanguageName::Arduino`.
//!
//! Imported from leachim6/hello-world (`a/Arduino.ino` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Arduino.ino>
//! - <https://www.wikidata.org/wiki/Q31092740> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Arduino => "Arduino",
    versions: [Arduino => "Arduino"],
    extensions: [Ino => "ino"],
}

const ARDUINO: &str = concat!(
    r#"void setup() {
  Serial.begin(9600);
  Serial.println(""#,
    output_for_hello_world!(),
    r#"");
}

void loop() {

}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arduino,
        extensions: &[FileExtension::Ino],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Procedural, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arduino,
            syntax: ARDUINO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ARDUINO,
                    [
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"setup"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  Serial",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"begin"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"9600"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
  Serial",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"loop"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };

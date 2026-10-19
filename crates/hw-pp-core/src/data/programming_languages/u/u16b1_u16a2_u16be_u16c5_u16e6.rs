//! Hello-world snippet for `ProgrammingLanguageName::U16b1U16a2U16beU16c5U16e6`.
//!
//! Imported from leachim6/hello-world (`#/ᚱᚢᚾᛅᛦ` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: U16b1U16a2U16beU16c5U16e6 => "ᚱᚢᚾᛅᛦ",
    versions: [U16b1U16a2U16beU16c5U16e6 => "ᚱᚢᚾᛅᛦ"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U16b1U16a2U16beU16c5U16e6,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U16b1U16a2U16beU16c5U16e6,
            syntax: r"ᛋᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭ
ᛁ͗̿ᛁ̊ᛁ̳͗̿ᛁ̹̊ᚹᛁ̿ᛁ̊ᛁ͗ᛁ̊ᛁ̹̊ᛁ̊̿ᛧ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"ᛋᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭ
ᛁ͗̿ᛁ̊ᛁ̳͗̿ᛁ̹̊ᚹᛁ̿ᛁ̊ᛁ͗ᛁ̊ᛁ̹̊ᛁ̊̿ᛧ",
            )],
        }],
    };

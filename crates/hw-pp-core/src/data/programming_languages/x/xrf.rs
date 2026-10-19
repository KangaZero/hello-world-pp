//! Hello-world snippet for `ProgrammingLanguageName::Xrf`.
//!
//! Imported from leachim6/hello-world (`x/XRF.xrf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Xrf => "XRF",
    versions: [Xrf => "XRF"],
    extensions: [Xrf => "xrf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xrf,
        extensions: &[FileExtension::Xrf],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xrf,
            syntax: r"53374 54374 54374 545F4 54374
54374 54374 41345 546F4 54374
54374 54374 54374 54554 54554
545F4 54314 54554 54554 54554
545F4 54314 54314 54554 545F4
54314 33745 54664 54664 54664
54664 54314 54664 54664 54664
54664 54664 54664 54314 54934
54554 5454F 54374 5451F 54314
54554 545F4 54314 54664 54664
54664 54314 54664 54664 54664
54664 9151B",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"53374 54374 54374 545F4 54374
54374 54374 41345 546F4 54374
54374 54374 54374 54554 54554
545F4 54314 54554 54554 54554
545F4 54314 54314 54554 545F4
54314 33745 54664 54664 54664
54664 54314 54664 54664 54664
54664 54664 54664 54314 54934
54554 5454F 54374 5451F 54314
54554 545F4 54314 54664 54664
54664 54314 54664 54664 54664
54664 9151B",
            )],
        }],
    };

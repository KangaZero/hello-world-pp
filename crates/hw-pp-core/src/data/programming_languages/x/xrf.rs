//! Hello-world snippet for `ProgrammingLanguageName::Xrf`.
//!
//! Imported from leachim6/hello-world (`x/XRF.xrf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XRF.xrf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xrf => "XRF",
    versions: [Xrf => "XRF"],
    extensions: [Xrf => "xrf"],
}

const XRF: &str = r"53374 54374 54374 545F4 54374
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
54664 9151B";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xrf,
        extensions: &[FileExtension::Xrf],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xrf,
            syntax: XRF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XRF,
                    [(
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
                )
            },
        }],
    };

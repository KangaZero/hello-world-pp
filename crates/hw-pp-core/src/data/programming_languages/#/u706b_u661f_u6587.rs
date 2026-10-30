//! Hello-world snippet for `ProgrammingLanguageName::U706bU661fU6587`.
//!
//! Imported from leachim6/hello-world (`#/火星文.martian` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E7%81%AB%E6%98%9F%E6%96%87.martian>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: U706bU661fU6587 => "火星文",
    versions: [U706bU661fU6587 => "火星文"],
    extensions: [Martian => "martian"],
}

const U706B_U661F_U6587: &str = r"姠屛募潑鎹牸苻賗：【沵恏，迣鎅】。
蔠圵姟珵垿。";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U706bU661fU6587,
        extensions: &[FileExtension::Martian],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U706bU661fU6587,
            syntax: U706B_U661F_U6587,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U706B_U661F_U6587,
                    [(
                        TokenKind::Plain,
                        r"姠屛募潑鎹牸苻賗：【沵恏，迣鎅】。
蔠圵姟珵垿。",
                    )],
                )
            },
        }],
    };

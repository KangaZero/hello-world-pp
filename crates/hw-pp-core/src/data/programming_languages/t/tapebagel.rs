//! Hello-world snippet for `ProgrammingLanguageName::Tapebagel`.
//!
//! Imported from leachim6/hello-world (`t/TapeBagel.tb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TapeBagel.tb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tapebagel => "TapeBagel",
    versions: [Tapebagel => "TapeBagel"],
    extensions: [Tb => "tb"],
}

const TAPEBAGEL: &str = r"%% %++ %++ %++ %++ %++ %++ %++ %++ @* ## %++ %++ %++ %++ %++ @* ## %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* @* ## %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* ## @* %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ %++ %++ %++ @* ##  %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ @* ##";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tapebagel,
        extensions: &[FileExtension::Tb],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tapebagel,
            syntax: TAPEBAGEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TAPEBAGEL,
                    [(
                        TokenKind::Plain,
                        r"%% %++ %++ %++ %++ %++ %++ %++ %++ @* ## %++ %++ %++ %++ %++ @* ## %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* @* ## %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* ## @* %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ %++ %++ %++ @* ##  %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ @* ##",
                    )],
                )
            },
        }],
    };

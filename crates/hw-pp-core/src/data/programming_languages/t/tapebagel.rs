//! Hello-world snippet for `ProgrammingLanguageName::Tapebagel`.
//!
//! Imported from leachim6/hello-world (`t/TapeBagel.tb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Tapebagel => "TapeBagel",
    versions: [Tapebagel => "TapeBagel"],
    extensions: [Tb => "tb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tapebagel,
        extensions: &[FileExtension::Tb],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tapebagel,
            syntax: r"%% %++ %++ %++ %++ %++ %++ %++ %++ @* ## %++ %++ %++ %++ %++ @* ## %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* @* ## %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ @* ## @* %++ %++ %++ %++
%++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ %++ %++ %++ @* ##  %++ %++ %++ %++ %++ %++ %++ %++ %++ %++
%++ %++ @* ## %++ %++ %++ %++ @* ##",
            tokenized_syntax: &[(
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
        }],
    };

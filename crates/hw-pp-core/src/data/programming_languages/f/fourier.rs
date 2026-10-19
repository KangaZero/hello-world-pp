//! Hello-world snippet for `ProgrammingLanguageName::Fourier`.
//!
//! Imported from leachim6/hello-world (`f/Fourier.fourier` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fourier.fourier>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Fourier => "Fourier",
    versions: [Fourier => "Fourier"],
    extensions: [Fourier => "fourier"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fourier,
        extensions: &[FileExtension::Fourier],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fourier,
            syntax: r"72a101a108aa111a44a32a119a111a114a108a100a33a10a",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"72a101a108aa111a44a32a119a111a114a108a100a33a10a",
            )],
        }],
    };

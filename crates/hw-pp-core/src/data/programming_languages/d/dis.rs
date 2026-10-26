//! Hello-world snippet for `ProgrammingLanguageName::Dis`.
//!
//! Imported from leachim6/hello-world (`d/Dis.dis` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dis.dis>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dis => "Dis",
    versions: [Dis => "Dis"],
    extensions: [Dis => "dis"],
}

const DIS: &str = r"^!|||||||||||||||||||||||||||||||||{!!!||||||||||||||||||||||||||||||||||||||||
||||||||||||||||*_>_*>|_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*>__*|_>*__|*|||*__
>*>||*>_|*__>*__>*__>*__>*__>*__>*__>*{_>*>_|*>||*__>*>||*>||*__>*__>*__>*>_|*>
||*__>*__>*__>*__>*__>*__>*{__*>||*>_|*__>*__>*>||*>||*__>*>_|*>||*__>*__>*__>*
__>*__>*__>*__>*{{>*>||*>_|*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>_|*>||*__>
*>_|*>||*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>||*>_|*>|>*>||*>_|*__>*__>*__
>*__>*__>*__>*__>*__>*{_>*>|_*>||*>_|*__>*>||*>_|*__>*__>*>_|*>||*__>*__>*__>*_
_>*__>*__>*{__*>||*>_|*__>*>_|*>||*__>*>|_*>||*>_|*__>*__>*__>*__>*__>*__>*__>*
__>*{_>*>|_*>||*>||*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>|_*>||*>_|*__>*__>
*__>*__>*__>*__>*__>*__>*__>*{_>*__|*>__|*>__|*__|*__>*__>*__>*__>*__>*__>*__>*
__>*__>*>_|*>||*{_>*__|*>__|*>__|*{_|*__>*>||*>||*{!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dis,
        extensions: &[FileExtension::Dis],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dis,
            syntax: DIS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DIS,
                    [(
                        TokenKind::Plain,
                        r"^!|||||||||||||||||||||||||||||||||{!!!||||||||||||||||||||||||||||||||||||||||
||||||||||||||||*_>_*>|_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*_>_*>__*|_>*__|*|||*__
>*>||*>_|*__>*__>*__>*__>*__>*__>*__>*{_>*>_|*>||*__>*>||*>||*__>*__>*__>*>_|*>
||*__>*__>*__>*__>*__>*__>*{__*>||*>_|*__>*__>*>||*>||*__>*>_|*>||*__>*__>*__>*
__>*__>*__>*__>*{{>*>||*>_|*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>_|*>||*__>
*>_|*>||*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>||*>_|*>|>*>||*>_|*__>*__>*__
>*__>*__>*__>*__>*__>*{_>*>|_*>||*>_|*__>*>||*>_|*__>*__>*>_|*>||*__>*__>*__>*_
_>*__>*__>*{__*>||*>_|*__>*>_|*>||*__>*>|_*>||*>_|*__>*__>*__>*__>*__>*__>*__>*
__>*{_>*>|_*>||*>||*__>*__>*__>*__>*__>*__>*__>*__>*__>*{_>*>|_*>||*>_|*__>*__>
*__>*__>*__>*__>*__>*__>*__>*{_>*__|*>__|*>__|*__|*__>*__>*__>*__>*__>*__>*__>*
__>*__>*>_|*>||*{_>*__|*>__|*>__|*{_|*__>*>||*>||*{!",
                    )],
                )
            },
        }],
    };

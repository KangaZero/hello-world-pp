//! Hello-world snippet for `ProgrammingLanguageName::N0815`.
//!
//! Imported from leachim6/hello-world (`#/0815.0815` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/0815.0815>
//! - <https://esolangs.org/wiki/0815> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::QueueBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N0815 => "0815",
    versions: [N0815 => "0815"],
    extensions: [E0815 => "0815"],
}

const N0815: &str = r"<:48:x<:65:=<:6C:$=$=$$~<:03:+
$~<:ffffffffffffffb1:+$<:77:~$
~<:fffffffffffff8:x+$~<:03:+$~
<:06:x-$x<:0e:x-$=x<:43:x-$";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0815,
        extensions: &[FileExtension::E0815],
        categories: &[Esolang],
        families: &[],
        paradigms: &[QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0815,
            syntax: N0815,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N0815,
                    [(
                        TokenKind::Plain,
                        r"<:48:x<:65:=<:6C:$=$=$$~<:03:+
$~<:ffffffffffffffb1:+$<:77:~$
~<:fffffffffffff8:x+$~<:03:+$~
<:06:x-$x<:0e:x-$=x<:43:x-$",
                    )],
                )
            },
        }],
    };

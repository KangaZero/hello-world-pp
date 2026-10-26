//! Hello-world snippet for `ProgrammingLanguageName::Acronym`.
//!
//! Imported from leachim6/hello-world (`a/ACRONYM.acronym` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ACRONYM.acronym>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Acronym => "ACRONYM",
    versions: [Acronym => "ACRONYM"],
    extensions: [Acronym => "acronym"],
}

const ACRONYM: &str = r"{{>>{~~~~{-<}~~~~~~~~~{-<-<}}<<}
</(<<<){[<]}:>:{>>{~~~~~~~~{<}~{>}}<<}\
~>{{~{v}}>>>v{~}^<<<}/(<<<){[<<]}:>:{>>{~~~~~~~~{<<}~{>>}}<<}\
~{>>{vvvvvvvv~~~~~~~~~~~~~~~}<<}~{>>{vvvv~~~~~~~~~~~~~}<<}~
{>>{^^^^^^^^^^~}<<}~{>>{v~~~~~~~{{<<}~}v{~}vvvvvvv{~{>>}}^^^^^^~~~
{{<<}~}vvvv{~{>>}}v~~~}<<}~<{{^^^}}~
{>>{vvvvvv~{{<<}~{>>}}^^^^^^^~~~~~~~~~~~{{<<}~{v>}}^^^^^^}}
/{{()}}{[<<<<]}:>:{{~v}}\}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acronym,
        extensions: &[FileExtension::Acronym],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acronym,
            syntax: ACRONYM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACRONYM,
                    [(
                        TokenKind::Plain,
                        r"{{>>{~~~~{-<}~~~~~~~~~{-<-<}}<<}
</(<<<){[<]}:>:{>>{~~~~~~~~{<}~{>}}<<}\
~>{{~{v}}>>>v{~}^<<<}/(<<<){[<<]}:>:{>>{~~~~~~~~{<<}~{>>}}<<}\
~{>>{vvvvvvvv~~~~~~~~~~~~~~~}<<}~{>>{vvvv~~~~~~~~~~~~~}<<}~
{>>{^^^^^^^^^^~}<<}~{>>{v~~~~~~~{{<<}~}v{~}vvvvvvv{~{>>}}^^^^^^~~~
{{<<}~}vvvv{~{>>}}v~~~}<<}~<{{^^^}}~
{>>{vvvvvv~{{<<}~{>>}}^^^^^^^~~~~~~~~~~~{{<<}~{v>}}^^^^^^}}
/{{()}}{[<<<<]}:>:{{~v}}\}",
                    )],
                )
            },
        }],
    };

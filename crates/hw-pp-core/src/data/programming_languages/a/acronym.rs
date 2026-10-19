//! Hello-world snippet for `ProgrammingLanguageName::Acronym`.
//!
//! Imported from leachim6/hello-world (`a/ACRONYM.acronym` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Acronym => "ACRONYM",
    versions: [Acronym => "ACRONYM"],
    extensions: [Acronym => "acronym"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acronym,
        extensions: &[FileExtension::Acronym],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acronym,
            syntax: r"{{>>{~~~~{-<}~~~~~~~~~{-<-<}}<<}
</(<<<){[<]}:>:{>>{~~~~~~~~{<}~{>}}<<}\
~>{{~{v}}>>>v{~}^<<<}/(<<<){[<<]}:>:{>>{~~~~~~~~{<<}~{>>}}<<}\
~{>>{vvvvvvvv~~~~~~~~~~~~~~~}<<}~{>>{vvvv~~~~~~~~~~~~~}<<}~
{>>{^^^^^^^^^^~}<<}~{>>{v~~~~~~~{{<<}~}v{~}vvvvvvv{~{>>}}^^^^^^~~~
{{<<}~}vvvv{~{>>}}v~~~}<<}~<{{^^^}}~
{>>{vvvvvv~{{<<}~{>>}}^^^^^^^~~~~~~~~~~~{{<<}~{v>}}^^^^^^}}
/{{()}}{[<<<<]}:>:{{~v}}\}",
            tokenized_syntax: &[(
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
        }],
    };

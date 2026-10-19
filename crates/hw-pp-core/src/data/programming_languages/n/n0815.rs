//! Hello-world snippet for `ProgrammingLanguageName::N0815`.
//!
//! Imported from leachim6/hello-world (`#/0815.0815` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: N0815 => "0815",
    versions: [N0815 => "0815"],
    extensions: [E0815 => "0815"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0815,
        extensions: &[FileExtension::E0815],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0815,
            syntax: r"<:48:x<:65:=<:6C:$=$=$$~<:03:+
$~<:ffffffffffffffb1:+$<:77:~$
~<:fffffffffffff8:x+$~<:03:+$~
<:06:x-$x<:0e:x-$=x<:43:x-$",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"<:48:x<:65:=<:6C:$=$=$$~<:03:+
$~<:ffffffffffffffb1:+$<:77:~$
~<:fffffffffffff8:x+$~<:03:+$~
<:06:x-$x<:0e:x-$=x<:43:x-$",
            )],
        }],
    };

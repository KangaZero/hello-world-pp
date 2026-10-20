//! Hello-world snippet for `ProgrammingLanguageName::Mineso`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Mineso>
//! - <https://esolangs.org/wiki/Mineso>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Mineso => "Mineso",
    versions: [Mineso => "Mineso"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mineso,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mineso,
            syntax: r"-1
<111111111111111111111111111111111111111111111111111111111111111111111111
>111111
<11111111111111111111111111111
>111111
<1111111
>111111
>111111
<111
>111111
=11111111111111111111111111111111
>111111
<1111111111111111111111111111111111111111111111111111111
>111111
<111111111111111111111111
>111111
<111
>111111
>11
-11
@111111
<111111
>111111
<11111111
>111111
=111111111111111111111111111111111
>111111",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"-1
<111111111111111111111111111111111111111111111111111111111111111111111111
>111111
<11111111111111111111111111111
>111111
<1111111
>111111
>111111
<111
>111111
=11111111111111111111111111111111
>111111
<1111111111111111111111111111111111111111111111111111111
>111111
<111111111111111111111111
>111111
<111
>111111
>11
-11
@111111
<111111
>111111
<11111111
>111111
=111111111111111111111111111111111
>111111",
            )],
        }],
    };

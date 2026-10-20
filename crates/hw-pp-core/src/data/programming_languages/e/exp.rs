//! Hello-world snippet for `ProgrammingLanguageName::Exp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Exp>
//! - <https://esolangs.org/wiki/Exp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Exp => "Exp",
    versions: [Exp => "Exp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Exp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Exp,
            syntax: r"{<|^^^^^^^^^| x |^^^^^^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^| - |^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| - |^|>}
{<|^^^^^^^^| x |^^^^| - |^|>}
{<|^^^^^^^^^^| x |^^^^^^^^^| - |^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| - |^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| + |^^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^^^^^^^^^| x |^^^^|>}",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"{<|^^^^^^^^^| x |^^^^^^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^| - |^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| - |^|>}
{<|^^^^^^^^| x |^^^^| - |^|>}
{<|^^^^^^^^^^| x |^^^^^^^^^| - |^^^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| - |^|>}
{<|^^^^^^^^^^| x |^^^^^^^^| + |^^|>}
{<|^^^^^^^^^| x |^^^^^^^^| + |^^^^|>}
{<|^^^^^^^^^^^^^^^^^| x |^^^^|>}",
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Dorklang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#dorklang>
//! - <https://esolangs.org/wiki/Dorklang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Dorklang => "dorklang",
    versions: [Dorklang => "dorklang"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dorklang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dorklang,
            syntax: r#"<pre class="rectwrap">''(')![+*+$:]!(+ +^*)-! !(+*+):!~++^$$:[++*][++/]!;:/!;:++[++/]!$;!+(+)+!--+!''/+!</pre>"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"<pre class="rectwrap">''(')![+*+$:]!(+ +^*)-! !(+*+):!~++^$$:[++*][++/]!;:/!;:++[++/]!$;!+(+)+!--+!''/+!</pre>"#,
            )],
        }],
    };

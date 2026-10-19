//! Hello-world snippet for `ProgrammingLanguageName::Zucchini`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Zucchini>
//! - <https://esolangs.org/wiki/Zucchini>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Zucchini => "Zucchini",
    versions: [Zucchini => "Zucchini"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zucchini,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zucchini,
            syntax: r"=0,1.2./.0,1./.3.17. ` /.4.17.`../.5.17.`: /.6.17.`: /.7.17.`::/.8.17`: /.9.17`  /.10.17.:.:/.11.17.`::/.12.17.: `/.13.17.`: /.14.17.`. /.15.17` ./.16.17``/.19.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"=0,1.2./.0,1./.3.17. ` /.4.17.`../.5.17.`: /.6.17.`: /.7.17.`::/.8.17`: /.9.17`  /.10.17.:.:/.11.17.`::/.12.17.: `/.13.17.`: /.14.17.`. /.15.17` ./.16.17``/.19.",
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::WikiSlashWikiSlashWikiSlashWikiSlashWikiSlashWikiSlash`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Wiki%2Fwiki%2Fwiki%2Fwiki%2Fwiki%2Fwiki%2F>
//! - <https://esolangs.org/wiki/Wiki%2Fwiki%2Fwiki%2Fwiki%2Fwiki%2Fwiki%2F>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: WikiSlashWikiSlashWikiSlashWikiSlashWikiSlashWikiSlash => "Wiki/wiki/wiki/wiki/wiki/wiki/",
    versions: [WikiSlashWikiSlashWikiSlashWikiSlashWikiSlashWikiSlash => "Wiki/wiki/wiki/wiki/wiki/wiki/"],
    extensions: [],
}

const WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH: &str = concat!(
    r"esolangs.org/w/wiki/w.php?title=",
    output_for_hello_world!(),
    r"/Talk:[[a]]"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::WikiSlashWikiSlashWikiSlashWikiSlashWikiSlashWikiSlash,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label:
                ProgrammingLanguageDialect::WikiSlashWikiSlashWikiSlashWikiSlashWikiSlashWikiSlash,
            syntax: WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH_WIKI_SLASH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"esolangs.org/w/wiki/w.php?title=",
                            output_for_hello_world!(),
                            r"/Talk:[[a]]"
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::StarLang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#%2Alang>
//! - <https://esolangs.org/wiki/%2Alang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: StarLang => "*lang",
    versions: [StarLang => "*lang"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StarLang,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StarLang,
            syntax: r"new *_;






























new * ;
new *!;










new *,;



























new *H;



























new *d;
new *e;






new *l;


new *o;


new *r;




new *w;
out H;
out e;
out l;
out l;
out o;
out ,;
out  ;
out w;
out o;
out r;
out l;
out d;
out !;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"new *_;






























new * ;
new *!;










new *,;



























new *H;



























new *d;
new *e;






new *l;


new *o;


new *r;




new *w;
out H;
out e;
out l;
out l;
out o;
out ,;
out  ;
out w;
out o;
out r;
out l;
out d;
out !;",
            )],
        }],
    };

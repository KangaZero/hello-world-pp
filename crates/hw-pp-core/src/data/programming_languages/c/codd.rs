//! Hello-world snippet for `ProgrammingLanguageName::Codd`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Codd>
//! - <https://esolangs.org/wiki/Codd>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Codd => "Codd",
    versions: [Codd => "Codd"],
    extensions: [],
}

const CODD: &str = r"λrx.r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[x]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] 

λqw.q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[w]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λvu.v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[u]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] 

λdf.d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[f]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λeg.e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[g]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λez.e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[z]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λpo.p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[o]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λmn.m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[n]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λzx.z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[x]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λjk.j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[k]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

`2`4`6`6`8`10`12`14`8`16`6`18`20";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Codd,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Codd,
            syntax: CODD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CODD,
                    [(
                        TokenKind::Plain,
                        r"λrx.r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[r[x]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] 

λqw.q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[q[w]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λvu.v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[v[u]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] 

λdf.d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[d[f]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λeg.e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[g]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λez.e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[e[z]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λpo.p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[p[o]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λmn.m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[m[n]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λzx.z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[z[x]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

λjk.j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[j[k]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]

`2`4`6`6`8`10`12`14`8`16`6`18`20",
                    )],
                )
            },
        }],
    };

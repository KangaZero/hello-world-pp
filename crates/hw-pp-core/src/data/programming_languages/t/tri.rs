//! Hello-world snippet for `ProgrammingLanguageName::Tri`.
//!
//! Imported from leachim6/hello-world (`t/Tri.tri` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tri.tri>
//! - <https://esolangs.org/wiki/Tri> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Tri => "Tri",
    versions: [Tri => "Tri"],
    extensions: [Tri => "tri"],
}

const TRI: &str = r"inc inc psh psh psh mul sto nxt sto psh mul lst psh add 
sto prn nxt zro clr inc inc inc psh psh add inc nxt pop 
inc psh lst psh mul one psh add sto nxt sto lst lst psh 
nxt nxt nxt add sto prn zro inc inc inc psh inc psh add 
sto prn prn inc inc inc prn clr lst lst lst psh nxt psh
sub nxt sto inc prn lst one clr inc psh psh inc psh mul 
nxt psh sub neg pop prn inc nxt psh nxt sto inc inc inc
inc inc inc inc inc prn lst prn nxt dec dec dec dec dec 
prn dec dec dec dec dec dec prn lst dec dec dec dec dec 
dec dec dec dec dec dec prn lst prn lin";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tri,
        extensions: &[FileExtension::Tri],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tri,
            syntax: TRI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TRI,
                    [(
                        TokenKind::Plain,
                        r"inc inc psh psh psh mul sto nxt sto psh mul lst psh add 
sto prn nxt zro clr inc inc inc psh psh add inc nxt pop 
inc psh lst psh mul one psh add sto nxt sto lst lst psh 
nxt nxt nxt add sto prn zro inc inc inc psh inc psh add 
sto prn prn inc inc inc prn clr lst lst lst psh nxt psh
sub nxt sto inc prn lst one clr inc psh psh inc psh mul 
nxt psh sub neg pop prn inc nxt psh nxt sto inc inc inc
inc inc inc inc inc prn lst prn nxt dec dec dec dec dec 
prn dec dec dec dec dec dec prn lst dec dec dec dec dec 
dec dec dec dec dec dec prn lst prn lin",
                    )],
                )
            },
        }],
    };

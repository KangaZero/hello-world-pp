//! Hello-world snippet for `ProgrammingLanguageName::Plankalkul`.
//!
//! Imported from the Hello World Collection (`files/Plankalk&uuml;l.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Plankalk%26uuml%3Bl.htm>
//! - <http://helloworldcollection.de/#Plankalk%C3%BCl>
//! - <https://www.wikidata.org/wiki/Q34280> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Plankalkul => "Plankalkül",
    versions: [Plankalkul => "Plankalkül"],
    extensions: [],
}

const PLANKALKUL: &str = r"R1.1(V0[:sig]) => R0
R1.2(V0[:m x sig]) => R0
0 => i | m + 1 => j
[W [ i  i ] ] ]
END
R1.3() => R0
'H';'e';'l';'l';'o';',';' ';'w';'o';'r';'l';'d';'!' => Z0[: m x sig] R1.2(Z0) => R0
END";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Plankalkul,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Plankalkul,
            syntax: PLANKALKUL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PLANKALKUL,
                    [(
                        TokenKind::Plain,
                        r"R1.1(V0[:sig]) => R0
R1.2(V0[:m x sig]) => R0
0 => i | m + 1 => j
[W [ i  i ] ] ]
END
R1.3() => R0
'H';'e';'l';'l';'o';',';' ';'w';'o';'r';'l';'d';'!' => Z0[: m x sig] R1.2(Z0) => R0
END",
                    )],
                )
            },
        }],
    };

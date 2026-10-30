//! Hello-world snippet for `ProgrammingLanguageName::Zetaplex`.
//!
//! Imported from leachim6/hello-world (`z/Zetaplex.zeta` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zetaplex.zeta>
//! - <https://esolangs.org/wiki/Zetaplex> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::TwoDimensional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Zetaplex => "Zetaplex",
    versions: [Zetaplex => "Zetaplex"],
    extensions: [Zeta => "zeta"],
}

const ZETAPLEX: &str = r"\      / #111#87\ #        #         ######
3      4 #        #        #        #      #
3      1 #        #        #        #      #
#      1 #        #        #        #      #
1####### /44#23#/ #        #        #      #
0      8 #        #        #        #      #
0      0 1        #        #        #      #
#      1 1        #        #        #      #
\      / \1#108sy 101#72Os DrFe####  ######";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zetaplex,
        extensions: &[FileExtension::Zeta],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zetaplex,
            syntax: ZETAPLEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZETAPLEX,
                    [(
                        TokenKind::Plain,
                        r"\      / #111#87\ #        #         ######
3      4 #        #        #        #      #
3      1 #        #        #        #      #
#      1 #        #        #        #      #
1####### /44#23#/ #        #        #      #
0      8 #        #        #        #      #
0      0 1        #        #        #      #
#      1 1        #        #        #      #
\      / \1#108sy 101#72Os DrFe####  ######",
                    )],
                )
            },
        }],
    };

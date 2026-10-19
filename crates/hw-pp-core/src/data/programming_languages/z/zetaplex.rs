//! Hello-world snippet for `ProgrammingLanguageName::Zetaplex`.
//!
//! Imported from leachim6/hello-world (`z/Zetaplex.zeta` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Zetaplex => "Zetaplex",
    versions: [Zetaplex => "Zetaplex"],
    extensions: [Zeta => "zeta"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zetaplex,
        extensions: &[FileExtension::Zeta],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zetaplex,
            syntax: r"\      / #111#87\ #        #         ######
3      4 #        #        #        #      #
3      1 #        #        #        #      #
#      1 #        #        #        #      #
1####### /44#23#/ #        #        #      #
0      8 #        #        #        #      #
0      0 1        #        #        #      #
#      1 1        #        #        #      #
\      / \1#108sy 101#72Os DrFe####  ######",
            tokenized_syntax: &[(
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
        }],
    };

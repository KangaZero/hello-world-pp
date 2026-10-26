//! Hello-world snippet for `ProgrammingLanguageName::Lola`.
//!
//! Imported from leachim6/hello-world (`l/LOLA.lola` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LOLA.lola>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lola => "LOLA",
    versions: [Lola => "LOLA"],
    extensions: [Lola => "lola"],
}

const LOLA: &str = r":H,:e,2:l,,:o,:',:_,:w,:o,:r,:l,:d,:!,:%,:\\b,\\a,,,,,,,,,,,,,,

+\\\cb,ba,,
*\\ba+,,\\a,
:\\\a\c,c,

H*+8,,8,
e+d,
l8+,d,
o3+,l,
'_+,^,
w8+,o,
r+^,+,e,
d2%,
_52,
!+_,

2\\bba,,
3+2,
5+22,,
832,
%*2,5,
^2*2,,3,";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lola,
        extensions: &[FileExtension::Lola],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lola,
            syntax: LOLA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOLA,
                    [(
                        TokenKind::Plain,
                        r":H,:e,2:l,,:o,:',:_,:w,:o,:r,:l,:d,:!,:%,:\\b,\\a,,,,,,,,,,,,,,

+\\\cb,ba,,
*\\ba+,,\\a,
:\\\a\c,c,

H*+8,,8,
e+d,
l8+,d,
o3+,l,
'_+,^,
w8+,o,
r+^,+,e,
d2%,
_52,
!+_,

2\\bba,,
3+2,
5+22,,
832,
%*2,5,
^2*2,,3,",
                    )],
                )
            },
        }],
    };

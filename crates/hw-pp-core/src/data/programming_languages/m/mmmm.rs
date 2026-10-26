//! Hello-world snippet for `ProgrammingLanguageName::Mmmm`.
//!
//! Imported from leachim6/hello-world (`m/Mmmm().mmmm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mmmm%28%29.mmmm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mmmm => "Mmmm()",
    versions: [Mmmm => "Mmmm()"],
    extensions: [Mmmm => "mmmm"],
}

const MMMM: &str = r"Mmm=m[m.m()].m(m.m());
Mmmm=m[mm].m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();Mmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();m[mmmm].m(mmm.m());
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();Mmmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
Mmmmmmmmm=mmm.m();
m[mmmm].m(mmm.m());
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();Mmmmmmm=mmm.m();
m[mmmm].m(mmmmmm);m[mmmm].m(mmmmmm);
mmm.m();mmm.m();
Mmmmmmmm=mmm.m();
m[mmmm].m(mmmmmmm);
mmm.m();mmm.m();
Mmmmmmmmmm=mmm.m();
Mmmm=m[mm].m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();
Mmmmmmmmmmm=mmm.m();
Mmmmmmmmmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();
m[mmmm].m(mmm.m());
m[mmmm].m(mmmmmmmmmm);
m[mmmm].m(mmmmm);
m[mmmm].m(mmmmmmm);
m[mmmm].m(mmmmmmmmm);
m[mmmm].m(mmmmmm);
m[mmmm].m(mmmmmmmm);
m[mmmm].m(mmmmmmmmmmm);";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mmmm,
        extensions: &[FileExtension::Mmmm],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mmmm,
            syntax: MMMM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MMMM,
                    [(
                        TokenKind::Plain,
                        r"Mmm=m[m.m()].m(m.m());
Mmmm=m[mm].m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();Mmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();m[mmmm].m(mmm.m());
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();Mmmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
Mmmmmmmmm=mmm.m();
m[mmmm].m(mmm.m());
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();Mmmmmmm=mmm.m();
m[mmmm].m(mmmmmm);m[mmmm].m(mmmmmm);
mmm.m();mmm.m();
Mmmmmmmm=mmm.m();
m[mmmm].m(mmmmmmm);
mmm.m();mmm.m();
Mmmmmmmmmm=mmm.m();
Mmmm=m[mm].m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();
Mmmmmmmmmmm=mmm.m();
Mmmmmmmmmmmm=mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();mmm.m();mmm.m();
mmm.m();mmm.m();
m[mmmm].m(mmm.m());
m[mmmm].m(mmmmmmmmmm);
m[mmmm].m(mmmmm);
m[mmmm].m(mmmmmmm);
m[mmmm].m(mmmmmmmmm);
m[mmmm].m(mmmmmm);
m[mmmm].m(mmmmmmmm);
m[mmmm].m(mmmmmmmmmmm);",
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::AnalfStarCk`.
//!
//! Imported from leachim6/hello-world (`a/AnalF∗ck.af` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AnalF%E2%88%97ck.af>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: AnalfStarCk => "AnalF∗ck",
    versions: [AnalfStarCk => "AnalF∗ck"],
    extensions: [Af => "af"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AnalfStarCk,
        extensions: &[FileExtension::Af],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AnalfStarCk,
            syntax: r"あくしろよ
いいよ！
アーッ！アーッ！
あのさぁ…あのさぁ…あのさぁ…あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
あくしろよ
来いよ！
アーッ！アーッ！あのさぁ…あのさぁ…あくしろよあくしろよ
あのさぁ…あくしろよ
ファッ！？
アーッ！アーッ！
あのさぁ…あくしろよ
ファッ！？
あのさぁ…あくしろよ
ファッ！？ファッ！？
あのさぁ…
ファッ！？
アーッ！アーッ！
アーッ！アーッ！
アーッ！アーッ！
あくしろよあくしろよあくしろよあくしろよ
あくしろよあくしろよあくしろよあくしろよ
ファッ！？
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
あのさぁ…あのさぁ…あのさぁ…あのさぁ…あのさぁ…
ファッ！？
アーッ！アーッ！
ファッ！？
あのさぁ…
ファッ！？
アーッ！アーッ！
あのさぁ…あくしろよ
あのさぁ…あくしろよ
ファッ！？
アーッ！アーッ！
あのさぁ…あのさぁ…
ファッ！？",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"あくしろよ
いいよ！
アーッ！アーッ！
あのさぁ…あのさぁ…あのさぁ…あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
アーッ！アーッ！
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あのさぁ…あのさぁ…あくしろよ
あくしろよ
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
あくしろよ
来いよ！
アーッ！アーッ！あのさぁ…あのさぁ…あくしろよあくしろよ
あのさぁ…あくしろよ
ファッ！？
アーッ！アーッ！
あのさぁ…あくしろよ
ファッ！？
あのさぁ…あくしろよ
ファッ！？ファッ！？
あのさぁ…
ファッ！？
アーッ！アーッ！
アーッ！アーッ！
アーッ！アーッ！
あくしろよあくしろよあくしろよあくしろよ
あくしろよあくしろよあくしろよあくしろよ
ファッ！？
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
ｱｰｯ!ｱｰｯ!ｱｰｯ!
あのさぁ…あのさぁ…あのさぁ…あのさぁ…あのさぁ…
ファッ！？
アーッ！アーッ！
ファッ！？
あのさぁ…
ファッ！？
アーッ！アーッ！
あのさぁ…あくしろよ
あのさぁ…あくしろよ
ファッ！？
アーッ！アーッ！
あのさぁ…あのさぁ…
ファッ！？",
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Yolang`.
//!
//! Imported from leachim6/hello-world (`y/YoLang.yo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/YoLang.yo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Yolang => "YoLang",
    versions: [Yolang => "YoLang"],
    extensions: [Yo => "yo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yolang,
        extensions: &[FileExtension::Yo],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yolang,
            syntax: r"Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!yo!yoYo!Yo!Yo!Yo!Yo!Yo!Yo!yoYo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!yoYo!Yo!Yo!yoYo!YOYOYOYOYo?YO?yoYo!Yo!YO!yoYo!YO!Yo!Yo!Yo!Yo!Yo!Yo!Yo!YO!YO!Yo!Yo!Yo!YO!yoYo!Yo!YO!YOYOYo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!YO!yoYO!Yo!Yo!Yo!YO!Yo?Yo?Yo?Yo?Yo?Yo?YO!Yo?Yo?Yo?Yo?Yo?Yo?Yo?Yo?YO!yoYo!YO!yoYO!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!yo!yoYo!Yo!Yo!Yo!Yo!Yo!Yo!yoYo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!yoYo!Yo!Yo!yoYo!YOYOYOYOYo?YO?yoYo!Yo!YO!yoYo!YO!Yo!Yo!Yo!Yo!Yo!Yo!Yo!YO!YO!Yo!Yo!Yo!YO!yoYo!Yo!YO!YOYOYo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!Yo!YO!yoYO!Yo!Yo!Yo!YO!Yo?Yo?Yo?Yo?Yo?Yo?YO!Yo?Yo?Yo?Yo?Yo?Yo?Yo?Yo?YO!yoYo!YO!yoYO!",
            )],
        }],
    };

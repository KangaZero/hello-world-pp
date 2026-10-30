//! Hello-world snippet for `ProgrammingLanguageName::Article`.
//!
//! Imported from leachim6/hello-world (`a/ARTICLE.artl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ARTICLE.artl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Article => "ARTICLE",
    versions: [Article => "ARTICLE"],
    extensions: [Artl => "artl"],
}

const ARTICLE: &str = r"add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1 n2 n3 n4. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1. w1 w2 w3 w4 w5 n1 n2 n3 n4 n6 n7 n8 n9 n10 n11 n12 n12 n13.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.
print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 w6 w7 w8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5 n1 n2 n3 n4. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8 n9
n10 n11 n12 n13 n14 n15.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. n1 n2
n3 n4 n5. n1 n2 n3 n4 n5 n6 n7 n8 n9 n10 n11 n12
n13.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Article,
        extensions: &[FileExtension::Artl],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Article,
            syntax: ARTICLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ARTICLE,
                    [(
                        TokenKind::Plain,
                        r"add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1 n2 n3 n4. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1. w1 w2 w3 w4 w5 n1 n2 n3 n4 n6 n7 n8 n9 n10 n11 n12 n12 n13.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.
print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 w6 w7 w8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5 n1 n2 n3 n4. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8 n9
n10 n11 n12 n13 n14 n15.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. n1 n2
n3 n4 n5. n1 n2 n3 n4 n5 n6 n7 n8 n9 n10 n11 n12
n13.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

add w2 w3 w4 w5 w6 w7 w8 w9 w10 w11. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.

subtract w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12. w1 w2 w3 w4 w5. w1 w2 w3 w4 w5 n1 n2 n3 n4 n5 n6 n7 n8.

print w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14.",
                    )],
                )
            },
        }],
    };

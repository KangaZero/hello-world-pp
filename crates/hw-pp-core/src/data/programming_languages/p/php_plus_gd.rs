//! Hello-world snippet for `ProgrammingLanguageName::PhpPlusGd`.
//!
//! Imported from the Hello World Collection (`files/PHP+GD.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PHP%2BGD.txt>
//! - <http://helloworldcollection.de/#PHP%2BGD>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PhpPlusGd => "PHP+GD",
    versions: [PhpPlusGd => "PHP+GD"],
    extensions: [],
}

const PHP_PLUS_GD: &str = concat!(
    r"<?
// ",
    output_for_hello_world!(),
    r#" in PHP + GD library
header("Content-type: image/gif");
$rscImage    = imagecreatetruecolor(80, 25);
$intFontC    = imagecolorallocate($rscImage, 255, 255, 255);
$intBGC        = imagecolorallocate($rscImage, 0, 0, 0);
imagestring($rscImage, 2, 5, 5, ""#,
    output_for_hello_world!(),
    r#"", $intFontC);
imagegif($rscImage);
imagedestroy($rscImage);
?>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PhpPlusGd,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PhpPlusGd,
            syntax: PHP_PLUS_GD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PHP_PLUS_GD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<?
// ",
                            output_for_hello_world!(),
                            r#" in PHP + GD library
header("Content-type: image/gif");
$rscImage    = imagecreatetruecolor(80, 25);
$intFontC    = imagecolorallocate($rscImage, 255, 255, 255);
$intBGC        = imagecolorallocate($rscImage, 0, 0, 0);
imagestring($rscImage, 2, 5, 5, ""#,
                            output_for_hello_world!(),
                            r#"", $intFontC);
imagegif($rscImage);
imagedestroy($rscImage);
?>"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::TaoPresentations`.
//!
//! Imported from leachim6/hello-world (`t/Tao Presentations.ddd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tao%20Presentations.ddd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: TaoPresentations => "Tao Presentations",
    versions: [TaoPresentations => "Tao Presentations"],
    extensions: [Ddd => "ddd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TaoPresentations,
        extensions: &[FileExtension::Ddd],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TaoPresentations,
            syntax: concat!(
                r"// ****************************************************************************
//  hello_world.ddd                                                 Tao project
// ****************************************************************************
//
// File Description:
//
// Original ",
                output_for_hello_world!(),
                r#" with Tao Presentations.
// http://www.taodyne.com/shop/en/blog/3-getting-started-with-tao-presentations
//
//
//
// ****************************************************************************
//  (C) 1992-2010 Christophe de Dinechin <christophe@taodyne.com>
//  (C) 2010 Jérôme Forissier <jerome@taodyne.com>
//  (C) 2010 Catherine Burvelle <cathy@taodyne.com>
//  (C) 2010 Taodyne SAS
// ****************************************************************************

color "white"
milkyway 10000
rotatez -23
earth 400
hello_world 440


milkyway R ->
// ----------------------------------------------------------------------------
//    Draw the Milky Way
// ----------------------------------------------------------------------------
    locally
        texture_wrap true, true
        texture_transform {scale 5, 5, 5}
        texture "milkyway.jpg"
        rotatey 0.02 * page_time + 100
        scale 1, -1, 1
        sphere R


earth R ->
// ----------------------------------------------------------------------------
//    Draw Earth
// ----------------------------------------------------------------------------
    locally
        texture "earth.bmp"
        texture_wrap true, true
        rotatey 5 * page_time + 250
        sphere 0, 0, 0, R


hello_world R ->
// ----------------------------------------------------------------------------
//    Draw ""#,
                output_for_hello_world!(),
                r#"" text
// ----------------------------------------------------------------------------
    locally
        frame_texture 1900, 600,
            color 1, 1, 1, 1
            reset_transform
            // If font Arial Unicode installed, it will be used.
            // Otherwise, unifont will be used (unifont is packaged
            // with Tao presentations).
            font "Arial Unicode MS", "unifont", 72
            move_to -800, -9, 0
            text ""#,
                output_for_hello_world!(),
                r#" or Καλημέρα κόσμε; or こんにちは 世界"
        rotatey -11 * page_time + 180
        color 20% , 20% , 20% , 70%
        sphere 0, 0, 0, R - 30
        color 100% , 90% , 20% , 90%
        sphere 0, 0, 0, R"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ****************************************************************************
//  hello_world.ddd                                                 Tao project
// ****************************************************************************
//
// File Description:
//
// Original ",
                    output_for_hello_world!(),
                    r#" with Tao Presentations.
// http://www.taodyne.com/shop/en/blog/3-getting-started-with-tao-presentations
//
//
//
// ****************************************************************************
//  (C) 1992-2010 Christophe de Dinechin <christophe@taodyne.com>
//  (C) 2010 Jérôme Forissier <jerome@taodyne.com>
//  (C) 2010 Catherine Burvelle <cathy@taodyne.com>
//  (C) 2010 Taodyne SAS
// ****************************************************************************

color "white"
milkyway 10000
rotatez -23
earth 400
hello_world 440


milkyway R ->
// ----------------------------------------------------------------------------
//    Draw the Milky Way
// ----------------------------------------------------------------------------
    locally
        texture_wrap true, true
        texture_transform {scale 5, 5, 5}
        texture "milkyway.jpg"
        rotatey 0.02 * page_time + 100
        scale 1, -1, 1
        sphere R


earth R ->
// ----------------------------------------------------------------------------
//    Draw Earth
// ----------------------------------------------------------------------------
    locally
        texture "earth.bmp"
        texture_wrap true, true
        rotatey 5 * page_time + 250
        sphere 0, 0, 0, R


hello_world R ->
// ----------------------------------------------------------------------------
//    Draw ""#,
                    output_for_hello_world!(),
                    r#"" text
// ----------------------------------------------------------------------------
    locally
        frame_texture 1900, 600,
            color 1, 1, 1, 1
            reset_transform
            // If font Arial Unicode installed, it will be used.
            // Otherwise, unifont will be used (unifont is packaged
            // with Tao presentations).
            font "Arial Unicode MS", "unifont", 72
            move_to -800, -9, 0
            text ""#,
                    output_for_hello_world!(),
                    r#" or Καλημέρα κόσμε; or こんにちは 世界"
        rotatey -11 * page_time + 180
        color 20% , 20% , 20% , 70%
        sphere 0, 0, 0, R - 30
        color 100% , 90% , 20% , 90%
        sphere 0, 0, 0, R"#
                ),
            )],
        }],
    };

//! Hello-world greetings for `HumanLanguageName::AmericanSignLanguage`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://www.lifeprint.com/asl101/pages-signs/h/hello.htm>
//! - <https://www.lifeprint.com/asl101/pages-signs/w/world.htm>
//! - <https://asl-transcriber.vercel.app/translate> (public-domain handshape drawings, via
//!   Wikimedia Commons)

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Signed,
    HumanLanguageFamily::FrenchSign, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: AmericanSignLanguage => "American Sign Language",
    dialects: [Asl => "ASL", LeftHandedAsl => "Left-handed ASL"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::AmericanSignLanguage,
    categories: &[Signed],
    families: &[FrenchSign],
    scripts: &[ScriptName::Fingerspelling],
    dialects: &[
        Dialect {
            label: DialectName::Asl,
            greetings: &[Greeting {
                scripts: &[ScriptName::Fingerspelling],
                // H-E-L-L-O W-O-R-L-D fingerspelled with the right hand, as ASCII art traced
                // from the handshape drawings (one character per cell, picked by glyph shape).
                text: r"             ___|||//\////|||||||_\__                       |/|/``\/\\             `\                  `\                  _||'\|_
        __||/'```    _||//''`    `'``\|__                  _`  |_  |  \            __                  __                _/_|__'\`\_
    _||/|\|        _/'`                '\|_               /``\| /|  |  \             /                   /             _|'/` _ '_|_|_
   ||`    |  ____||'                     `|_             | |_ __-|/\|-|         _.|  |              _.|  |            ||/`  /`\__/||/
  _|   ___||/|\|``'|_     __|||//          \|_           |  `\_|||\/\/|     __/`` |-.|          __/`` |-.|            _`   |_  _|``|
_|/'\||\|'`  \||__||`__|/|'|  `|_           ||\\\|||_|   |.   \|/|     |   |` | / |  `   _     |` | / |  `   _        |    | '|`' _(
|`      |     ||||||/'`    |\__|`\_       _/`       `    \_     /''|. _/   || | _.|  \//``'|   || | _.|  \//``'|      |    )  ` |/`
\|______|||//||'|``     __|||||\|_`\\||_|/'               \_   |   ` _/    |/\/./-/    _./'`   |/\/./-/    _./'`      \     \  /
 `''''``        \|___|/|``   '\||\                         \   \_   |'     |    |  _|/`        |    |  _|/`          |     | _/
                 `\|'`    ___|//`          __               \_  `'\\`       ___  ''|            ___  ''|            _`    '|/`
                   ||_ _||'`            __|/`               | `     _       ``     (            ``     (            /
                    '||''\\\/////////||'''|\\||||||        |`       \      /       |           /       |           /      '

  _  ''  _.|           _||'\|_           |`|              `\                 /`\
 /`| --_ |_|         _/_|__'\`\_        |`\|              __                 |.|
 | '   | ` |       _|'/` _ '_|_|_       '_.'_               /                |  _
 )  _\ ||  `      ||/`  /`\__/||/      __|_-'          _.|  |            __/\|  |
 _\||| || |       _`   |_  _|``|      |/\||. \     __/`` |-.|          _|`|  |--|
 |'/ `\   |       |    | '|`' _(     /\|' `\  |   |` | / |  `   _     |` \ |_|  |
|_`\\  \_ \       |    )  ` |/`     |/_'||  \_|   || | _.|  \//``'|   |||)/|||_ |
|`'/|\_ '_\       \     \  /        | '\|`'  |/   |/\/./-/    _./'`   |```'|(''_\
|     `  |       |     | _/         |   |    |    |    |  _|/`        '_  | '-'|
|        (      _`    '|/`           \  |   _|     ___  ''|            |_ '___|'
 |      |       /                     \. \         ``     (             ``  ``_
 |      \      /      '                .          /       |             /     |",
            }],
        },
        Dialect {
            label: DialectName::LeftHandedAsl,
            greetings: &[Greeting {
                scripts: &[ScriptName::Fingerspelling],
                // The same handshapes mirrored, as signed with the left hand.
                text: r"                 __/_|||||||\\\\/\\|||___                  /\\/``\|/|             /`                  /`              _|/'||_
             __|/``'``   `''\\||_    ```'\||__            /  |  _/  __            __                  __            _/`/'__|_\_
           _|/'                 '\_        |/|\||_       /  |` |\ |/``\_         |                   |             _|_|_' _  \'|_
          _|`                     '||____  |    `|\       |/|/\|-__ _| |         |  \._              |  \._        \|||__/`\  `\||
        _|/          \\|||__     _|' `|/|\||___   |_      |\/\/|||_/`  |         |.-| '`\__          |.-| '`\__     | `|_  _|   `_
|_|||///||           _|`  |'|\|__'||__||/  `'|/||/'\|_   |`    |\|/   .|     _      | - | `|     _      | - | `|    |_ ' |/ |    |
 ``      '\_       _/`|__//    `'\\|||||     |      '|   \_ .|/'\`    _/   |'``\\/  |._ | ||   |'``\\/  |._ | ||      \| `  (    |
           '\|_||//`_|/|||||__      `|'||\\|||______|/    \_ `   |   _/    `'\.__   \-(.\/\|   `'\.__   \-(.\/\|        \        '
                    |||/`   ``|\|___|/        ``''''`      '|   _/   /          `\|_  |    |        `\|_  |    |         \_ \     |
         __          `\\|___     '|/`                       '/\'`  _/              |''` ___`           |''` ___`          `\|'    '_
          ||__            `'||_ _||                         _     ` |              |     `'            |     `'              _     )
   ||||||//|'''|\\\\\\\\\\///''\|'                          /        |             |       \           |       \             \      \

|._  /'  _        _|/'||_              |`|               /`            /`\
|_| _-- |`\     _/`/'__|_\_            |/`|              __            \.|
| ` |   ' |    _|_|_' _  \'|_         _'._`             |             _` |
'  ||  _  (    \|||__/`\  `\||        (- | _            |  \._        |  \/\__
 | || |||/_     | `|_  _|   `_       / ./|/\|           |.-| '`\__    |--|  | |_
 |   /` \'|     |_ ' |/ |    |      |  |` '|/\      _      | - | `|   /  |_| |  |
 ' _/  //`_|      \| `  (    |      |_/  ||'_\|   |'``\\/  |._ | ||   | _/||(/|/|
  _` _/|\'`|        \        '      ||  '`||` |   `'\.__   \-(.\/\|   '_`'\|'```|
  |  `     |         \_ \     |      |    |   /        `\|_  |    |    |'-' |  _\
  )        (          `\|'    '_     |_   |  |            |''` ___`    '|___' _|
   |   `  |              _     )         / ./             |     `'      _``  ```
   |      |              \      \          .              |       \     |     \",
            }],
        },
    ],
};

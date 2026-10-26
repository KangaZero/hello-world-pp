//! Hello-world snippet for `ProgrammingLanguageName::Velato`.
//!
//! Imported from leachim6/hello-world (`v/Velato.ly` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Velato.ly>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Velato => "Velato",
    versions: [Velato => "Velato"],
    extensions: [],
}

const VELATO: &str = concat!(
    r#"on "2.19.64"

\header{
  title = ""#,
    output_for_hello_world!(),
    r#""
}

\language "english"

\paper {
  top-margin = 1.5\in
  left-margin = 2.0\in
  right-margin = 2.0\in
}

\score {
  \transpose c c' {
    % H
    <c a>4
    g8(
    e8 f8
    gs8 d8
    g8) |

    % e
    a8
    <g e>4.
    <f cs>4
    c16( cs16
    g8 |

    % l
    a8)
    <g e>8
    <f cs>4
    <c a>4
    g4( |

    % l
    a16 g16
    e16 f8
    cs8.) <c a g>2 |

    % o
    a8( g8 e8)
    f8.
    cs16( cs8 cs8 g8 |

    % ,
    a8 g4
    e8 f8
    e e
    g8) |

    % space
    a8
    <g e>4.
    f8(
    ds d
    g8 |

    % W
    a8 g4
    e8 f8
    a8 gs8
    g8) |

    % o
    a8( g8
    e8 f8
    cs8 cs8) <cs g>4 |

    % r
    a8( g8
    e8 f8
    cs8 cs8 e8. g16 |

    % l
    c16 a16)
    <g e>8
    <f cs>4
    <c a>4
    g4( |

    % d
    a8 g4
    e16 f16
    cs8 c8 c8
    g8 |

    % !
    a16)
    <g e>4
    f16( ds16 ds16)
    <g c>2 |
  }
  \layout {
    indent = 0\cm
  }
  \midi {
    \tempo 4 = 120
  }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Velato,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Velato,
            syntax: VELATO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VELATO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"on "2.19.64"

\header{
  title = ""#,
                            output_for_hello_world!(),
                            r#""
}

\language "english"

\paper {
  top-margin = 1.5\in
  left-margin = 2.0\in
  right-margin = 2.0\in
}

\score {
  \transpose c c' {
    % H
    <c a>4
    g8(
    e8 f8
    gs8 d8
    g8) |

    % e
    a8
    <g e>4.
    <f cs>4
    c16( cs16
    g8 |

    % l
    a8)
    <g e>8
    <f cs>4
    <c a>4
    g4( |

    % l
    a16 g16
    e16 f8
    cs8.) <c a g>2 |

    % o
    a8( g8 e8)
    f8.
    cs16( cs8 cs8 g8 |

    % ,
    a8 g4
    e8 f8
    e e
    g8) |

    % space
    a8
    <g e>4.
    f8(
    ds d
    g8 |

    % W
    a8 g4
    e8 f8
    a8 gs8
    g8) |

    % o
    a8( g8
    e8 f8
    cs8 cs8) <cs g>4 |

    % r
    a8( g8
    e8 f8
    cs8 cs8 e8. g16 |

    % l
    c16 a16)
    <g e>8
    <f cs>4
    <c a>4
    g4( |

    % d
    a8 g4
    e16 f16
    cs8 c8 c8
    g8 |

    % !
    a16)
    <g e>4
    f16( ds16 ds16)
    <g c>2 |
  }
  \layout {
    indent = 0\cm
  }
  \midi {
    \tempo 4 = 120
  }
}"#
                        ),
                    )],
                )
            },
        }],
    };

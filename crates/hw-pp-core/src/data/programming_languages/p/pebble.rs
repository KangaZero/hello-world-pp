//! Hello-world snippet for `ProgrammingLanguageName::Pebble`.
//!
//! Imported from leachim6/hello-world (`p/Pebble.c` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pebble.c>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Pebble => "Pebble",
    versions: [Pebble => "Pebble"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pebble,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pebble,
            syntax: r#"#include <pebble.h>

static Window* main_window;
static TextLayer* text_layer, *world_layer;

static void main_window_load(Window* window) {
    Layer *window_layer = window_get_root_layer(window);
    window_set_background_color(main_window, GColorBlack);
    GRect bounds = layer_get_bounds(window_layer);
    text_layer = text_layer_create(
        GRect(0, bounds.size.h / 2 - 42, bounds.size.w, 120));
    text_layer_set_text(text_layer, "Hello");
    text_layer_set_background_color(text_layer, GColorClear);
    text_layer_set_text_color(text_layer, GColorWhite);
    text_layer_set_font(text_layer, fonts_get_system_font(FONT_KEY_BITHAM_42_BOLD));
    text_layer_set_text_alignment(text_layer, GTextAlignmentCenter);

    world_layer = text_layer_create(
        GRect(0, bounds.size.h / 2, bounds.size.w, 120));
    text_layer_set_background_color(world_layer, GColorClear);
    text_layer_set_text(world_layer, "World");
    text_layer_set_text_color(world_layer, GColorLightGray);
    text_layer_set_font(world_layer, fonts_get_system_font(FONT_KEY_BITHAM_42_BOLD));
    text_layer_set_text_alignment(world_layer, GTextAlignmentCenter);
  
    layer_add_child(window_layer, text_layer_get_layer(text_layer));
    layer_add_child(window_layer, text_layer_get_layer(world_layer));
}
static void main_window_unload(Window* window) {
    text_layer_destroy(text_layer);
    text_layer_destroy(world_layer);
}

static void init() {
    main_window = window_create();
    window_set_window_handlers(main_window, (WindowHandlers) {
        .load = main_window_load,
        .unload = main_window_unload
    });
    window_stack_push(main_window, true);
}

static void deinit() {
    window_destroy(main_window);
}

int main(void) {
    init();
    app_event_loop();
    deinit();
}"#,
            tokenized_syntax: &[
                (TokenKind::Keyword, r"#include"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"<pebble.h>"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" Window"),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r" main_window"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" TextLayer"),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r" text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main_window_load"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"Window"),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r" window"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    Layer ",
                ),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r"window_layer "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"window_get_root_layer"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"window"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"window_set_background_color"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"main_window"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GColorBlack"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    GRect bounds ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"layer_get_bounds"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"window_layer"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    text_layer ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"text_layer_create"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Function, r"GRect"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" bounds"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"size"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"h "),
                (TokenKind::Keyword, r"/"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"2"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"-"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"42"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" bounds"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"size"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"w"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"120"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""Hello""#),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_background_color"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GColorClear"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text_color"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GColorWhite"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_font"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"fonts_get_system_font"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"FONT_KEY_BITHAM_42_BOLD"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text_alignment"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GTextAlignmentCenter"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"

    world_layer ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"text_layer_create"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Function, r"GRect"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" bounds"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"size"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"h "),
                (TokenKind::Keyword, r"/"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"2"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" bounds"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"size"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"w"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"120"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_background_color"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GColorClear"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""World""#),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text_color"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GColorLightGray"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_font"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"fonts_get_system_font"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"FONT_KEY_BITHAM_42_BOLD"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_set_text_alignment"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" GTextAlignmentCenter"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
  
    ",
                ),
                (TokenKind::Function, r"layer_add_child"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"window_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"text_layer_get_layer"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"layer_add_child"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"window_layer"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"text_layer_get_layer"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r"));"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main_window_unload"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"Window"),
                (TokenKind::Keyword, r"*"),
                (TokenKind::Plain, r" window"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_destroy"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text_layer"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"text_layer_destroy"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"world_layer"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"init"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    main_window ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"window_create"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"window_set_window_handlers"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"main_window"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"WindowHandlers"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"load "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" main_window_load"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"unload "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" main_window_unload
    ",
                ),
                (TokenKind::Punctuation, r"});"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"window_stack_push"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"main_window"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" true"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"deinit"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"window_destroy"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"main_window"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Type, r"int"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"void"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"init"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"app_event_loop"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"deinit"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };

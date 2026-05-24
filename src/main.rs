use ply_engine::prelude::*;

const INIT_WINDOW_W: u16 = 100;
const INIT_WINDOW_H: u16 = 50;
const MAX_LINE_LENGTH: u16 = 20;
const FONT_SIZE: u16 = 16;
const CHAR_WIDTH: u16 = 6; // approximate for monospace at size 16; TODO: check based on measure_text?
const LINE_HEIGHT: u16 = FONT_SIZE + 6;
const MARGIN_OFFSET: u16 = 16;
const BG_COLOUR: u32 = 0x1E1B1B;
const MAIN_TEXT_COLOUR: u32 = 0xE8E0DC;
const PLACEHOLDER_TEXT_COLOUR: u32 = 0x6E6560;
const CURSOR_COLOUR: u32 = 0xFF654D;
const SELECTION_COLOUR: (u8, u8, u8, u8) = (255, 101, 77, 51); // TODO: standardise to u32
// const SELECTION_COLOUR: u32 = 0x334D65FF; // or 0xFF654D33 in BE

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "scratch".to_owned(),
            window_width: INIT_WINDOW_W as i32,
            window_height: INIT_WINDOW_H as i32,
            high_dpi: true,
            sample_count: 4,
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 100000,
        ..Default::default()
    }
}

fn measure_with_font(
    text: &str,
    config: &ply_engine::text::TextConfig,
) -> ply_engine::math::Dimensions {
    let dims = macroquad::text::measure_text(text, None, config.font_size, 1.0);
    ply_engine::math::Dimensions {
        width: dims.width,
        height: dims.height,
    }
}

fn desired_window_size(line_count: usize) -> (f32, f32) {
    let w = (MAX_LINE_LENGTH * CHAR_WIDTH + MARGIN_OFFSET * 2).max(100) as f32;
    let h = (line_count as u16 * LINE_HEIGHT + MARGIN_OFFSET * 2) as f32;
    (w, h)
}

fn calculate_window_size(text: &str) -> (f32, f32) {
    let lines = ply_engine::text_input::wrap_lines(
        text,
        (MAX_LINE_LENGTH * CHAR_WIDTH) as f32,
        None,
        FONT_SIZE,
        &measure_with_font,
    );
    desired_window_size(lines.len())
}

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/julia-mono.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(BLACK);

        let mut ui = ply.begin();

        let text = ui.get_text_value("editor").to_string();

        if is_key_pressed(KeyCode::Escape) {
            on_close(&text);
            std::process::exit(0);
        }

        let (w, h) = calculate_window_size(&text);
        macroquad::window::request_new_screen_size(w, h);

        ui.element()
            .width(grow!())
            .height(grow!())
            .background_color(BG_COLOUR)
            .layout(|l| l.padding(MARGIN_OFFSET))
            .children(|ui| {
                ui.element()
                    .id("editor")
                    .width(grow!())
                    .height(grow!())
                    .text_input(|t| {
                        t.font_size(FONT_SIZE)
                            .line_height(LINE_HEIGHT)
                            .text_color(MAIN_TEXT_COLOUR)
                            .placeholder("Start typing...")
                            .placeholder_color(PLACEHOLDER_TEXT_COLOUR)
                            .cursor_color(CURSOR_COLOUR)
                            .selection_color(SELECTION_COLOUR)
                            .drag_select()
                            .multiline()
                    })
                    .empty();
            });

        ui.show(|_| {}).await;

        next_frame().await;
    }
}

fn on_close(text: &str) {
    // TODO: clipboard
    println!("{}", text);
}

use ply_engine::prelude::*;

const MARGIN_OFFSET: u16 = 16;
const FONT_SIZE: u8 = 16;
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
            window_width: 800,
            window_height: 600,
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

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/julia-mono.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(BLACK);

        let mut ui = ply.begin();

        if is_key_pressed(KeyCode::Escape) {
            let text = ui.get_text_value("editor").to_string();
            on_close(&text);
            std::process::exit(0);
        }

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
                            .text_color(MAIN_TEXT_COLOUR)
                            .placeholder("Start typing...")
                            .placeholder_color(PLACEHOLDER_TEXT_COLOUR)
                            .cursor_color(CURSOR_COLOUR)
                            .selection_color(SELECTION_COLOUR)
                            .drag_select()
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

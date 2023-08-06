use std::env;

use piston_window::{clear, Context, G2d, Glyphs, PistonWindow, Position, rectangle, text, TextureContext, TextureSettings, Transformed, WindowSettings};
use piston_window::types::Color;

pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub fn main() {
    let _args: Vec<_> = env::args().collect();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let ref font = assets.join("retro-gaming.ttf");

    let mut window: PistonWindow = WindowSettings::new("test", [800, 600])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(
        font,
        TextureContext {
            factory,
            encoder: window.factory.create_command_buffer().into(),
        },
        TextureSettings::new(),
    ).unwrap();


    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, device| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      ctx.transform, g);
            // draw_text(&ctx, g, &mut glyphs, TEXT_COLOR, Pos { x: 0.0, y: 20.0 }, "301211");
            text::Text::new_color(TEXT_COLOR, 20)
                .draw(
                    "10iubuoiygbiuyg87tr85riuygiu1011123456789111",
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(0.0, 20.0),
                    g,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
}

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub fn draw_text(
    ctx: &Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    color: Color,
    pos: Pos,
    text: &str,
) {
    text::Text::new_color(color, 20)
        .draw(
            text,
            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(pos.x as f64, pos.y as f64),
            graphics,
        )
        .unwrap();
}

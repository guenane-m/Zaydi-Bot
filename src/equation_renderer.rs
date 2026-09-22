use ratex_layout::{layout, to_display_list, LayoutOptions};
use ratex_parser::parse;
use ratex_render::{render_to_png, RenderOptions};
use ratex_types::color::Color;

pub fn render(latex: &str) -> Vec<u8> {
    let nodes = parse(latex).map_err(|e| e.to_string()).unwrap();

    let layout_options = LayoutOptions::default();
    let layout_box = layout(&nodes, &layout_options);
    let display_list = to_display_list(&layout_box);

    // Create and populate render options:
    let render_options = RenderOptions {
        font_size: 40.0,                            // Set font size.
        padding: 10.0,                              // Add padding.
        background_color: Color::WHITE,             // Set background color.
        font_dir: String::new(),                    // unused when embed-fonts is on.
        device_pixel_ratio: 2.0,                    // render at 2× for sharper Telegram photos.
    };

    render_to_png(&display_list, &render_options)   // Render the equation as PNG and return it.
        .expect("failed to render png")
}
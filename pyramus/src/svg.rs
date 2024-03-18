// Creates a simple SVG tree with a rectangle
// TODO: This is for testing purposes only
// Alpha is a value between 0.0 and 1.0
pub fn build_svg_rect(w: u32, h: u32, color: &str, stroke: Option<u32>, alpha: f32) -> String {
    let stroke = match stroke {
        Some(stroke) => format!(
            r#"stroke="{color}" stroke-width="{stroke}" fill="{color}" fill-opacity="{alpha}"#,
        ),
        None => format!(r#"fill="{color}" fill-opacity="{alpha}"#),
    };

    format!(
        r#"
            <svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
                <rect x="0" y="0" width="{w}" height="{h}" {stroke}" />
            </svg>
            "#
    )
}

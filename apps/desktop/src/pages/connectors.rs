use gpui_kit::*;

pub fn render() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(rgb(0x0F1115))
        .p_6()
        .child(
            div()
                .text_color(rgb(0xFFFFFF))
                .child("This is the Connectors Page"),
        )
}

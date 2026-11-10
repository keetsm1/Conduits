use gpui_kit::*;

use crate::components::card::connector_card;

pub fn render() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(rgb(0x0F1115))
        .p_6()
        .gap_4()
        .child(connector_card(
            "Google",
            "Connect Google Drive and Gmail to your conduits.",
        ))
}

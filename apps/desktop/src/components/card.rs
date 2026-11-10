use gpui_kit::component::v_flex;
use gpui_kit::component::Icon;
use gpui_kit::assets::IconName;
use gpui_kit::*;

pub fn connector_card(name: &str, description: &str) -> impl IntoElement {
    let button_id = SharedString::from(format!("connect-{}", name));

    v_flex()
        .w(px(240.))
        .gap_4()
        .p_4()
        .bg(rgb(0x161920))
        .rounded(px(14.))
        .border_1()
        .border_color(rgb(0x262B36))
        //logo tile
        .child(
            v_flex()
                .w_full()
                .h(px(110.))
                .items_center()
                .justify_center()
                .bg(rgb(0x1D212A))
                .rounded(px(10.))
                .child(
                    Icon::new(IconName::Plug)
                        .size(px(28.))
                        .text_color(rgb(0x9EA4B0)),
                ),
        )
        .child(
            v_flex()
                .w_full()
                .gap_1()
                .items_center()
                //connector_name:
                .child(
                    div()
                        .text_lg()
                        .text_color(rgb(0xFFFFFF))
                        .child(name.to_string()),
                )
                .child(
                    div()
                        .w_full()
                        .text_center()
                        .text_sm()
                        .text_color(rgb(0x9EA4B0))
                        .child(description.to_string()),
                ),
        )
        //connect btn
        .child(
            div()
                .id(button_id)
                .w_full()
                .py_1p5()
                .rounded(px(8.))
                .bg(rgb(0x3B82F6))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|this| this.bg(rgb(0x2563EB)))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xFFFFFF))
                        .child("Connect"),
                ),
        )
}

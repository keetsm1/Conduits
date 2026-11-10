use gpui_kit::assets::IconName;
use gpui_kit::component::Icon;
use gpui_kit::component::Sizable as _;
use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::v_flex;
use gpui_kit::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardStatus {
    Idle,
    Connecting,
    Connected,
    Failed,
}

pub fn connector_card(
    name: &str,
    description: &str,
    status: CardStatus,
    on_connect: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let button_id = SharedString::from(format!("connect-{}", name));

    let (bg, hover, label): (u32, u32, &str) = match status {
        CardStatus::Idle => (0x3B82F6, 0x2563EB, "Connect"),
        CardStatus::Connecting => (0x3A404D, 0x3A404D, "Connecting"),
        CardStatus::Connected => (0x16A34A, 0x16A34A, "Connected"),
        CardStatus::Failed => (0xDC2626, 0xDC2626, "Failed"),
    };

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
                .bg(rgb(bg))
                .flex()
                .items_center()
                .justify_center()
                .gap_2()
                .cursor_pointer()
                .hover(move |this| this.bg(rgb(hover)))
                .on_click(move |_, window, cx| on_connect(window, cx))
                // spinner only exists while connecting
                .children((status == CardStatus::Connecting).then(|| {
                    Spinner::new()
                        .with_size(px(14.))
                        .color(Hsla::from(rgb(0xFFFFFF)))
                }))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xFFFFFF))
                        .child(label),
                ),
        )
}

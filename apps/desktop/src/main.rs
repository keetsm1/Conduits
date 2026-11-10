use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::component::TitleBar;
use gpui_kit::component::v_flex;
use gpui_kit::*;

mod components {
    pub mod card;
    pub mod sidebar;
}

mod pages {
    pub mod connectors;
}

mod helpers{
    pub mod connect_google;
}

use components::sidebar::{Page, sidebar};

struct HelloWorld {
    current_page: Page,
    connectors: Entity<pages::connectors::ConnectorsPage>,
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        div()
            .flex()
            .size_full()
            .bg(rgb(0x0F1115))
            .text_color(rgb(0xFFFFFF))
            // The sidebar runs full height, so the strip behind the traffic lights is
            // sidebar-colored and the title bar only sits over the main column.
            .child(
                v_flex()
                    .w(px(240.))
                    .h_full()
                    .flex_shrink_0()
                    .child(
                        div()
                            .id("traffic-light-band")
                            .w_full()
                            .h(px(34.))
                            .flex_shrink_0()
                            .bg(rgb(0x12141A))
                            .border_r_1()
                            .border_color(rgb(0x262B36))
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.start_window_move()
                            }),
                    )
                    .child(
                        div().flex_1().min_h_0().child(sidebar(
                            self.current_page,
                            move |page, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.current_page = page;
                                    cx.notify()
                                });
                            },
                        )),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    // Same fill as the page and no divider: the bar disappears into the app.
                    .child(
                        // Painted explicitly: theme tokens get replaced by the default theme
                        // load, which left the bar darker than the page.
                        TitleBar::new()
                            .bg(rgb(0x0F1115))
                            .border_color(rgb(0x0F1115)),
                    )
                    .child(
                        div().flex_1().child(match self.current_page {
                            Page::Dashboard => "Hello, World!".into_any_element(),
                            Page::Connectors => self.connectors.clone().into_any_element(),
                        }),
                    ),
            )
    }
}

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| {
            init(cx);

            // Keep the native window chrome (titlebar, border) in the app's dark theme.
            cx.set_window_appearance(Some(WindowAppearance::Dark));

            // The sidebar paints its own active/hover colors from these tokens.
            Theme::change(ThemeMode::Dark, None, cx);
            let theme = Theme::global_mut(cx);
            theme.tokens.sidebar = Hsla::from(rgb(0x12141A)).into();
            theme.tokens.sidebar_foreground = Hsla::from(rgb(0x8A8F9D)).into();
            theme.tokens.sidebar_accent = Hsla::from(rgb(0x232833)).into();
            theme.tokens.sidebar_accent_foreground = Hsla::from(rgb(0xFFFFFF)).into();
            theme.tokens.sidebar_border = Hsla::from(rgb(0x262B36)).into();

            // Transparent native title bar with traffic lights over our own drawn bar.
            open_window(TitleBar::window_options(), cx, |_, cx| {
                cx.new(|cx| HelloWorld {
                    current_page: Page::Dashboard,
                    connectors: cx.new(|_| pages::connectors::ConnectorsPage::new()),
                })
            })
            .expect("Failed to open window");
        });
}

use gpui_kit::component::theme::{Theme, ThemeMode};
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
            .child(sidebar(self.current_page, move |page, _, cx| {
                entity.update(cx, |this, cx| {
                    this.current_page = page;
                    cx.notify()
                });
            }))
            .child(
                div().flex_1().child(match self.current_page {
                    Page::Dashboard => "Hello, World!".into_any_element(),
                    Page::Connectors => self.connectors.clone().into_any_element(),
                }),
            )
    }
}

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| {
            init(cx);

            // The sidebar paints its own active/hover colors from these tokens.
            Theme::change(ThemeMode::Dark, None, cx);
            let theme = Theme::global_mut(cx);
            theme.tokens.sidebar = Hsla::from(rgb(0x12141A)).into();
            theme.tokens.sidebar_foreground = Hsla::from(rgb(0x8A8F9D)).into();
            theme.tokens.sidebar_accent = Hsla::from(rgb(0x232833)).into();
            theme.tokens.sidebar_accent_foreground = Hsla::from(rgb(0xFFFFFF)).into();
            theme.tokens.sidebar_border = Hsla::from(rgb(0x262B36)).into();

            open_window(WindowOptions::default(), cx, |_, cx| {
                cx.new(|cx| HelloWorld {
                    current_page: Page::Dashboard,
                    connectors: cx.new(|_| pages::connectors::ConnectorsPage::new()),
                })
            })
            .expect("Failed to open window");
        });
}

use gpui_kit::*;

mod components {
    pub mod sidebar;
}

mod pages {
    pub mod connectors;
}

use components::sidebar::{Page, sidebar};

struct HelloWorld {
    current_page: Page,
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
                    Page::Connectors => pages::connectors::render().into_any_element(),
                }),
            )
    }
}

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| {
            init(cx);

            open_window(WindowOptions::default(), cx, |_, cx| {
                cx.new(|_| HelloWorld {
                    current_page: Page::Dashboard,
                })
            })
            .expect("Failed to open window");
        });
}

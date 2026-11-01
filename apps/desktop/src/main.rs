use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .child("Hello, World!")
            .child(
                Button::new("Connect Gmail")
                    .primary()
                    .label("Click me")
                    .on_click(|_, _, _| println!("Connected Gmail!")),
            )
            .child(
                div()
                    .flex()
                    .size_full()
                    .justify_end()
                    .child(
                        Button::new("Outlook")
                        .primary()
                        .label("Connect to Outlook")
                        .on_click(|_, _, _| println!("Connected Outlook!")),
                    )
            )
    }
}

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| {
            init(cx);

            open_window(WindowOptions::default(), cx, |_, cx| {
                cx.new(|_| HelloWorld)
            })
            .expect("Failed to open window");
        });
}
use std::time::Duration;

use gpui_kit::component::WindowExt as _;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;

use crate::components::card::{CardStatus, connector_card};
use crate::helpers::connect_google::connect_google;

const FAILED_TOAST_LIFETIME: Duration = Duration::from_secs(10);

/// Type id used to remove the error toast after its countdown.
struct ConnectFailedToast;

pub struct ConnectorsPage {
    status: CardStatus,
}

impl ConnectorsPage {
    pub fn new() -> Self {
        Self {
            status: CardStatus::Idle,
        }
    }

    fn connect(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.status != CardStatus::Idle {
            return;
        }

        self.status = CardStatus::Connecting;
        cx.notify();

        // Blocking HTTP on a background thread: gpui's executor has no tokio reactor.
        let request = cx.background_spawn(async move { connect_google() });

        cx.spawn_in(window, async move |this, cx| {
            let result = request.await;

            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(_) => this.status = CardStatus::Connected,
                    Err(error) => {
                        this.status = CardStatus::Failed;
                        window.push_notification(
                            Notification::error(format!("Google connection failed: {error}"))
                                .id::<ConnectFailedToast>()
                                .placement(Anchor::BottomCenter)
                                .autohide(false),
                            cx,
                        );

                        // Toast outlives the button state: dismiss both together.
                        cx.spawn_in(window, async move |this, cx| {
                            cx.background_executor().timer(FAILED_TOAST_LIFETIME).await;
                            this.update_in(cx, |this, window, cx| {
                                window.remove_notification::<ConnectFailedToast>(cx);
                                this.status = CardStatus::Idle;
                                cx.notify();
                            })
                            .ok();
                        })
                        .detach();
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for ConnectorsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
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
                self.status,
                move |window, cx| {
                    entity.update(cx, |this, cx| this.connect(window, cx));
                },
            ))
    }
}

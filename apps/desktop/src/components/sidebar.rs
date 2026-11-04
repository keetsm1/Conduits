use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_kit::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Dashboard,
    Connectors,
}

pub fn sidebar(
    current: Page,
    on_navigate: impl Fn(Page, &mut Window, &mut App) + 'static,
) -> Sidebar<SidebarGroup<SidebarMenu>> {
    
    let on_navigate = Rc::new(on_navigate);


    let nav = |page: Page, label: &'static str, icon: IconName| {
        let on_navigate = on_navigate.clone();
        SidebarMenuItem::new(label)
            .icon(icon)
            .active(current == page)
            .on_click(move |_, window, cx| on_navigate(page, window, cx))
    };

    Sidebar::new("main-sidebar")
        .w(px(240.))
        .bg(rgb(0x12141A))
        .text_color(rgb(0x8A8F9D))
        .border_color(rgb(0x262B36))
        .header(SidebarHeader::new().child("Conduits"))
        .child(
            SidebarGroup::new("Workspace").child(
                SidebarMenu::new()
                    .child(nav(Page::Dashboard, "Dashboard", IconName::LayoutDashboard))
                    .child(SidebarMenuItem::new("AI Agents").icon(IconName::Bot))
                    .child(SidebarMenuItem::new("Automations").icon(IconName::Settings2))
                    .child(SidebarMenuItem::new("Database").icon(IconName::Folder))
                    .child(nav(Page::Connectors, "Connectors", IconName::ExternalLink)),
            ),
        )
        .footer(SidebarFooter::new().child("Settings"))
}

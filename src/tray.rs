use anyhow::{Context, Result};
use std::collections::HashMap;
use indexmap::IndexMap;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, MenuId, PredefinedMenuItem, Submenu};
use tray_icon::{TrayIcon, TrayIconBuilder};

use crate::models::{ProxyGroup, TrayEvent};

pub struct TrayManager {
    tray_icon: TrayIcon,
    menu: Menu,
    menu_id_map: HashMap<MenuId, String>,
}

impl TrayManager {
    pub fn new() -> Result<Self> {
        // Create main menu
        let menu = Menu::new();

        // Create tray icon with default icon
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("Clash Node Switcher")
            .with_icon(Self::create_default_icon()?)
            .build()
            .context("Failed to create tray icon")?;

        Ok(TrayManager {
            tray_icon,
            menu,
            menu_id_map: HashMap::new(),
        })
    }

    pub fn update_menu(&mut self, proxy_groups: &IndexMap<String, ProxyGroup>, node_delays: &IndexMap<String, u32>, green_threshold: u32, yellow_threshold: u32) -> Result<()> {
        // Clear existing menu ID map
        self.menu_id_map.clear();
        let new_menu = Menu::new();

        // Add proxy groups as submenus
        for (group_name, group) in proxy_groups {
            if let Some(ref nodes) = group.all {
                // Create submenu for this group
                let submenu = Submenu::new(group_name, true);

                // Add all nodes to the submenu
                for node_name in nodes {
                    let is_current = group.now.as_ref() == Some(node_name);

                    // Build menu text with delay and color indicator if available
                    let menu_text = if let Some(&delay) = node_delays.get(node_name) {
                        // Determine color indicator based on delay
                        let color_indicator = if delay < green_threshold {
                            "🟢" // Green
                        } else if delay < yellow_threshold {
                            "🟡" // Yellow
                        } else {
                            "🔴" // Red
                        };

                        if is_current {
                            format!("✓ {} {} {}ms", node_name, color_indicator, delay)
                        } else {
                            format!("{} {} {}ms", node_name, color_indicator, delay)
                        }
                    } else {
                        if is_current {
                            format!("✓ {}", node_name)
                        } else {
                            node_name.clone()
                        }
                    };

                    let menu_item = MenuItem::new(menu_text, true, None);

                    // Store menu ID mapping for event handling
                    let item_key = format!("node::{}::{}", group_name, node_name);
                    self.menu_id_map.insert(menu_item.id().clone(), item_key);

                    submenu.append(&menu_item).ok();
                }

                // Append the submenu to the main menu
                new_menu.append(&submenu).ok();
            }
        }

        // Add separator
        new_menu.append(&PredefinedMenuItem::separator()).ok();

        // Add refresh option
        let refresh_item = MenuItem::new("Refresh", true, None);
        self.menu_id_map.insert(refresh_item.id().clone(), "refresh".to_string());
        new_menu.append(&refresh_item).ok();

        // Add quit option
        let quit_item = MenuItem::new("Quit", true, None);
        self.menu_id_map.insert(quit_item.id().clone(), "quit".to_string());
        new_menu.append(&quit_item).ok();

        // Update tray icon menu
        self.menu = new_menu.clone();
        self.tray_icon.set_menu(Some(Box::new(new_menu)));

        Ok(())
    }

    pub fn set_icon_connected(&mut self) -> Result<()> {
        self.tray_icon.set_icon(Some(Self::create_connected_icon()?))?;
        Ok(())
    }

    pub fn set_icon_disconnected(&mut self) -> Result<()> {
        self.tray_icon.set_icon(Some(Self::create_disconnected_icon()?))?;
        Ok(())
    }

    pub fn poll_events(&mut self) -> Option<TrayEvent> {
        // Check for menu events
        if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            if let Some(action_key) = self.menu_id_map.get(&menu_event.id) {
                if action_key == "refresh" {
                    return Some(TrayEvent::Refresh);
                } else if action_key == "quit" {
                    return Some(TrayEvent::Quit);
                } else if action_key.starts_with("node::") {
                    // Parse: "node::group_name::node_name"
                    let parts: Vec<&str> = action_key.split("::").collect();
                    if parts.len() == 3 {
                        return Some(TrayEvent::SwitchNode {
                            group: parts[1].to_string(),
                            node: parts[2].to_string(),
                        });
                    }
                }
            }
        }

        None
    }

    fn create_default_icon() -> Result<tray_icon::Icon> {
        Self::load_icon_from_bytes(include_bytes!("../assets/icon.png"))
    }

    fn create_connected_icon() -> Result<tray_icon::Icon> {
        Self::load_icon_from_bytes(include_bytes!("../assets/icon.png"))
    }

    fn create_disconnected_icon() -> Result<tray_icon::Icon> {
        Self::load_icon_from_bytes(include_bytes!("../assets/icon.png"))
    }

    fn load_icon_from_bytes(bytes: &[u8]) -> Result<tray_icon::Icon> {
        let img = image::load_from_memory(bytes)
            .context("Failed to load icon from bytes")?
            .to_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        tray_icon::Icon::from_rgba(rgba, width, height)
            .context("Failed to create icon from RGBA data")
    }
}

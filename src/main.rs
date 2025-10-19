mod clash_api;
mod config;
mod models;
mod tray;

use anyhow::{Context, Result};
use clash_api::ClashApi;
use config::Config;
use models::TrayEvent;
use notify_rust::Notification;
use std::collections::HashMap;
use std::time::Duration;
use tray::TrayManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize GTK (required for tray-icon)
    gtk::init().context("Failed to initialize GTK")?;

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    println!("Clash Node Switcher (clbar) starting...");
    println!("Config loaded from: ~/.config/clbar/config.toml");
    println!("Clash API URL: {}", config.clash_api_url);

    // Initialize Clash API client
    let clash_api = ClashApi::new(&config).context("Failed to create Clash API client")?;

    // Test connection
    if let Err(e) = clash_api.test_connection().await {
        eprintln!("Failed to connect to Clash API: {}", e);
        notify_error("Connection Failed", &format!("Cannot connect to Clash API: {}", e));
        std::process::exit(1);
    }

    println!("Connected to Clash API successfully");

    // Initialize tray icon
    let mut tray_manager = TrayManager::new().context("Failed to create tray manager")?;

    // Initial proxy groups fetch
    let mut proxy_groups = fetch_proxy_groups(&clash_api, &config).await?;
    tray_manager.update_menu(&proxy_groups)?;
    tray_manager.set_icon_connected()?;

    println!("Tray icon initialized");

    // Event loop
    let mut last_refresh = std::time::Instant::now();
    let refresh_interval = Duration::from_secs(config.refresh_interval_secs);

    loop {
        // Poll tray events
        while let Some(event) = tray_manager.poll_events() {
            match event {
                TrayEvent::SwitchNode { group, node } => {
                    println!("Switching {} to {}", group, node);
                    handle_switch_node(&clash_api, &mut tray_manager, &group, &node).await;
                    // Refresh proxy groups after switch
                    if let Ok(groups) = fetch_proxy_groups(&clash_api, &config).await {
                        proxy_groups = groups;
                        tray_manager.update_menu(&proxy_groups)?;
                    }
                }
                TrayEvent::Refresh => {
                    println!("Refreshing proxy groups...");
                    match fetch_proxy_groups(&clash_api, &config).await {
                        Ok(groups) => {
                            proxy_groups = groups;
                            tray_manager.update_menu(&proxy_groups)?;
                            tray_manager.set_icon_connected()?;
                            notify_success("Refreshed", "Proxy groups updated successfully");
                        }
                        Err(e) => {
                            eprintln!("Failed to refresh: {}", e);
                            notify_error("Refresh Failed", &format!("Error: {}", e));
                            tray_manager.set_icon_disconnected()?;
                        }
                    }
                }
                TrayEvent::Quit => {
                    println!("Quitting...");
                    std::process::exit(0);
                }
            }
        }

        // Auto-refresh proxy groups periodically
        if last_refresh.elapsed() >= refresh_interval {
            if let Ok(groups) = fetch_proxy_groups(&clash_api, &config).await {
                proxy_groups = groups;
                tray_manager.update_menu(&proxy_groups)?;
                tray_manager.set_icon_connected()?;
            } else {
                tray_manager.set_icon_disconnected()?;
            }
            last_refresh = std::time::Instant::now();
        }

        // Process GTK events (non-blocking)
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        // Small sleep to prevent busy loop
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn fetch_proxy_groups(
    clash_api: &ClashApi,
    config: &Config,
) -> Result<HashMap<String, models::ProxyGroup>> {
    let all_groups = clash_api.get_proxies().await?;

    // Filter to only configured groups if specified
    if config.proxy_groups.is_empty() {
        return Ok(all_groups);
    }

    let filtered_groups: HashMap<String, models::ProxyGroup> = all_groups
        .into_iter()
        .filter(|(name, _)| config.proxy_groups.contains(name))
        .collect();

    Ok(filtered_groups)
}

async fn handle_switch_node(
    clash_api: &ClashApi,
    tray_manager: &mut TrayManager,
    group: &str,
    node: &str,
) {
    match clash_api.switch_node(group, node).await {
        Ok(_) => {
            println!("Successfully switched {} to {}", group, node);
            notify_success(
                "Node Switched",
                &format!("Switched {} to {}", group, node),
            );
            tray_manager.set_icon_connected().ok();
        }
        Err(e) => {
            eprintln!("Failed to switch node: {}", e);
            notify_error(
                "Switch Failed",
                &format!("Failed to switch {}: {}", group, e),
            );
            tray_manager.set_icon_disconnected().ok();
        }
    }
}

fn notify_success(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(3000)
        .show()
        .ok();
}

fn notify_error(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .urgency(notify_rust::Urgency::Critical)
        .timeout(5000)
        .show()
        .ok();
}

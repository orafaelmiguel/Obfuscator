use eframe::egui;
use crate::state::{ObscuraState, AuthMsg};
use crate::auth_client;
use std::sync::mpsc;

/// Centered login card with cleaner layout and no direct Margin usage.
pub fn show_login(ui: &mut egui::Ui, state: &mut ObscuraState) {
    // Use a centered top-down layout so the card sits in the middle of the panel.
    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        // Outer spacing to push the card away from edges
        ui.add_space(24.0);

        // Draw a framed card. We avoid calling `inner_margin(...)` directly to prevent
        // Margin / type mismatches across egui versions.
        egui::Frame::default()
            .rounding(egui::Rounding::same(8))
            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.inactive.bg_fill))
            .show(ui, |ui| {
                // Limit the card width so it looks like a centered form
                let desired_width = 420.0_f32;
                ui.set_max_width(desired_width);

                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    ui.heading("Obscura Defender");
                    ui.small("Please log in to continue");
                    ui.add_space(16.0);

                    // Input box group
                    ui.vertical(|ui| {
                        ui.label("Email");
                        ui.add(egui::TextEdit::singleline(&mut state.email).desired_width(desired_width - 40.0));

                        ui.add_space(8.0);

                        ui.label("Password");
                        ui.add(
                            egui::TextEdit::singleline(&mut state.password)
                                .password(true)
                                .desired_width(desired_width - 40.0)
                                .hint_text("••••••••"),
                        );
                    });

                    ui.add_space(14.0);

                    // Buttons row
                    ui.horizontal(|ui| {
                        if state.auth_processing {
                            ui.add_enabled(false, egui::Button::new("Logging in...").min_size([160.0, 30.0].into()));
                        } else if ui.add_sized([160.0, 30.0], egui::Button::new("Login")).clicked() {
                            // start auth thread
                            let email = state.email.clone();
                            let password = state.password.clone();
                            let base_url = std::env::var("OBSCURA_API_URL")
                                .unwrap_or_else(|_| "https://api.obscurasec.io".to_string());

                            let (tx, rx) = mpsc::channel();

                            state.auth_processing = true;
                            state.auth_rx = Some(rx);
                            state.push_log("Starting authentication request...");

                            std::thread::spawn(move || {
                                match auth_client::login_request(&base_url, &email, &password) {
                                    Ok(resp) => {
                                        let _ = tx.send(AuthMsg::Success(resp.accessToken));
                                    }
                                    Err(e) => {
                                        let msg = match e {
                                            auth_client::AuthError::Status(code, body) => {
                                                format!("HTTP {}: {}", code, body)
                                            }
                                            auth_client::AuthError::Deserialize(s) => {
                                                format!("Invalid response: {}", s)
                                            }
                                            auth_client::AuthError::Network(s) => {
                                                format!("Network error: {}", s)
                                            }
                                            auth_client::AuthError::Http(s) => s,
                                        };
                                        let _ = tx.send(AuthMsg::Error(msg));
                                    }
                                }
                            });
                        }

                        ui.add_space(8.0);

                        if ui.button("Help").clicked() {
                            state.push_log("Help requested (placeholder)");
                        }
                    });

                    ui.add_space(8.0);

                    // Inline error message area (red)
                    if let Some(err) = &state.last_auth_error {
                        ui.colored_label(egui::Color32::from_rgb(220, 60, 60), format!("Error: {}", err));
                        ui.add_space(6.0);
                    }

                    ui.add_space(8.0);
                });
                // end card inner UI
            });
        // optional: extra bottom spacing
        ui.add_space(16.0);
    });
}

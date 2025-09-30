use eframe::egui;
use crate::state::{ObscuraState, AuthMsg};
use crate::auth_client;
use std::sync::mpsc;

pub fn show_login(ui: &mut egui::Ui, state: &mut ObscuraState) {
    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        let card_width = 540.0; // mais largo
        let input_height = 30.0;
        let button_height = 30.0;

        egui::Frame::default()
            .corner_radius(16) // cantos mais suaves
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                ui.set_width(card_width);

                ui.vertical_centered(|ui| {
                    // Título principal
                    ui.heading(
                        egui::RichText::new("Obscura Defender")
                            .size(36.0)
                            .strong(),
                    );

                    // Subtítulo
                    ui.label(
                        egui::RichText::new("Secure your applications with ease")
                            .color(ui.visuals().weak_text_color())
                            .size(20.0)
                            .italics(),
                    );

                    ui.add_space(40.0);

                    // Campo Email
                    ui.label(
                        egui::RichText::new("Email").size(18.0).color(ui.visuals().text_color()),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut state.email)
                            .hint_text("you@example.com")
                            .desired_width(card_width - 80.0)
                            .min_size([card_width - 80.0, input_height].into()),
                    );

                    ui.add_space(20.0);

                    // Campo Password
                    ui.label(
                        egui::RichText::new("Password").size(18.0).color(ui.visuals().text_color()),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut state.password)
                            .password(true)
                            .hint_text("••••••••")
                            .desired_width(card_width - 80.0)
                            .min_size([card_width - 80.0, input_height].into()),
                    );

                    ui.add_space(30.0);

                    // Botão de Login
                    if state.auth_processing {
                        ui.add_enabled(
                            false,
                            egui::Button::new("Logging in...")
                                .min_size([card_width - 80.0, button_height].into()),
                        );
                    } else if ui
                        .add_sized([card_width - 80.0, button_height], egui::Button::new("Login"))
                        .clicked()
                    {
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

                    ui.add_space(20.0);

                    // Mensagem de erro
                    if let Some(err) = &state.last_auth_error {
                        ui.colored_label(
                            egui::Color32::from_rgb(220, 60, 60),
                            egui::RichText::new(format!("Error: {}", err)).size(16.0),
                        );
                        ui.add_space(10.0);
                    }

                    // Footer
                    ui.hyperlink_to(
                        egui::RichText::new("Forgot Password")
                            .color(ui.visuals().hyperlink_color)
                            .underline()
                            .size(16.0),
                        "https://obscurasec.io/forgot-password"
                    )
                });
            });
    });
}

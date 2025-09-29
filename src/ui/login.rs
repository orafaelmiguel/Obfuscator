use eframe::egui;
use crate::state::ObscuraState;
use crate::auth_client;
use std::sync::mpsc;
use std::path::PathBuf;

pub fn show_login(ui: &mut egui::Ui, state: &mut ObscuraState) {
    ui.heading("Login");

    ui.label("Email:");
    ui.text_edit_singleline(&mut state.email);

    ui.label("Password:");
    ui.add(egui::TextEdit::singleline(&mut state.password).password(true));

    // show spinner or disabled state while auth is running
    if state.auth_processing {
        ui.label("Logging in...");
    }

    if ui.add_enabled(!state.auth_processing, egui::Button::new("Login")).clicked() {
        // start auth thread
        let email = state.email.clone();
        let password = state.password.clone();
        // base url: could be config; for now hardcode or use env var
        let base_url = std::env::var("OBSCURA_API_URL").unwrap_or_else(|_| "https://api.obscurasec.io".to_string());

        let (tx, rx) = mpsc::channel();

        // mark processing and set rx
        state.auth_processing = true;
        state.auth_rx = Some(rx);
        state.push_log("Starting authentication request...");

        std::thread::spawn(move || {
            match auth_client::login_request(&base_url, &email, &password) {
                Ok(resp) => {
                    let _ = tx.send(crate::state::AuthMsg::Success(resp.accessToken));
                }
                Err(e) => {
                    let msg = match e {
                        crate::auth_client::AuthError::Status(code, body) => {
                            format!("HTTP {}: {}", code, body)
                        }
                        crate::auth_client::AuthError::Deserialize(s) => format!("Invalid response: {}", s),
                        crate::auth_client::AuthError::Network(s) => format!("Network error: {}", s),
                        crate::auth_client::AuthError::Http(s) => s,
                    };
                    let _ = tx.send(crate::state::AuthMsg::Error(msg));
                }
            }
        });
    }

    // Allow pressing Enter? optional
}

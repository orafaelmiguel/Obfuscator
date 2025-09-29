use crate::state::{AppState, ObscuraState};

/// Mock login: accepts any email/password.
/// Returns true if login successful, false otherwise.
pub fn login(state: &mut ObscuraState) -> bool {
    if state.email.trim().is_empty() || state.password.trim().is_empty() {
        state.push_log("Login failed: empty email or password");
        return false;
    }

    // In the future: call API here
    state.push_log(format!("User {} logged in (mock)", state.email));
    state.state = AppState::Dashboard;
    true
}

/// Mock logout
pub fn logout(state: &mut ObscuraState) {
    state.push_log(format!("User {} logged out", state.email));
    state.email.clear();
    state.password.clear();
    state.state = AppState::Login;
}

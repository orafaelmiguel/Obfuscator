use crate::state::{AppState, ObscuraState};

pub fn login(state: &mut ObscuraState) -> bool {
    if state.email.trim().is_empty() || state.password.trim().is_empty() {
        state.push_log("Login failed: empty email or password");
        return false;
    }

    state.push_log(format!("User {} logged in (mock)", state.email));
    state.state = AppState::Dashboard;
    true
}

pub fn logout(state: &mut ObscuraState) {
    state.push_log(format!("User {} logged out", state.email));
    state.email.clear();
    state.password.clear();
    state.state = AppState::Login;
}

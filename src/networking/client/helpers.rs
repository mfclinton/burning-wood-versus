use turbo::*;

pub fn is_client_player(user_id: &str) -> bool {
    turbo::os::client::user_id().map_or(false, |client_id| client_id == user_id)
}
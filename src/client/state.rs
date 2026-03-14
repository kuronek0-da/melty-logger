use rand::RngExt;

#[derive(Debug,Clone)]
pub enum ClientState {
    Idle, // Not in ranked
    HostingRanked(String), // Hosting, not in match yet
    JoinedRanked(String), // Joined, not in match yet
    MatchInProgress(String), // In match
}

impl ClientState {
    pub fn hosting() -> ClientState {
        ClientState::HostingRanked(ClientState::generate_match_code())
    }

    pub fn join(session_id: String) -> ClientState {
        ClientState::JoinedRanked(session_id)
    }

    pub fn get_session(&self) -> Option<&str> {
        match self {
            ClientState::HostingRanked(s) => Some(&s),
            ClientState::JoinedRanked(s) => Some(&s),
            ClientState::MatchInProgress(s) => Some(&s),
            _ => None
        }
    }

    fn generate_match_code() -> String {
        let mut rng = rand::rng();
        (0..6).map(|_| rng.sample(rand::distr::Alphanumeric) as char)
            .collect::<String>()
            .to_uppercase()
    }
}
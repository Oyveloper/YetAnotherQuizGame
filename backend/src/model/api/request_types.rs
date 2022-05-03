pub struct ConnectRequest {
    pub username: String,
    pub game_id: String,
}

pub struct DisConnectRequest {
    pub username: String,
    pub game_id: String,
}

pub struct CreateGameRequest {
    pub username: String,
    pub quiz_id: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub roles: [String; 3],
    pub uuid: String,
    pub jti: String,
    pub platform: String,
}

fn stringify_jwt(jwt: &JwtPayload) -> String {
    format!(
        "users:development:{}:{}:{}:{}",
        jwt.roles.join(";"),
        jwt.uuid,
        jwt.jti,
        jwt.platform
    )
}

pub fn get_jwt_fixtures() -> Vec<String> {
    let valid_token = &JwtPayload {
        roles: [
            String::from("content.client"),
            String::from("therapySession.client"),
            String::from("videoChat.client"),
        ],
        uuid: String::from("ecf0fb38-1fe9-11e8-a296-0242ac110002"),
        jti: String::from("63547718-63e7-43cb-8143-ec80c8e68df7"),
        platform: String::from("web"),
    };
    let invalid_token = &JwtPayload {
        roles: [
            String::from("content.client"),
            String::from("therapySession.client"),
            String::from("videoChat.client"),
        ],
        uuid: String::from("ddb1a8af-90d2-45be-9459-da410606553e"),
        jti: String::from("42d14c82-739b-4c60-bdaa-94ba5b7de00a"),
        platform: String::from("web"),
    };
    let jwts: Vec<&JwtPayload> = vec![valid_token, invalid_token];
    jwts.into_iter()
        .map(|jwt: &JwtPayload| stringify_jwt(jwt))
        .collect()
}

use std::time::SystemTime;
use warp::Reply;

use crate::errors::WebResult;
use crate::jwt::get_jwt_fixtures;
use crate::pool::{set_str, MobcPool};

pub async fn load_fixtures(pool: MobcPool) -> WebResult<impl Reply> {
    let epoch_duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let epoch = &*epoch_duration.as_secs().to_string();
    for jwt in get_jwt_fixtures() {
        debug!("inserting: key: {}; value: {}", jwt, epoch);
        set_str(&pool, &jwt, &epoch, 0)
            .await
            .map_err(warp::reject::custom)?;
    }
    Ok("fixtures loaded")
}

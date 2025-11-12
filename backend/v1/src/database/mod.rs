#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use sqlx::{Executor, PgConnection};

pub mod pool;

const LOAD_SAMPLE_SQL: &str = include_str!("../../sample/sample.up.sql");
const UNLOAD_SAMPLE_SQL: &str = include_str!("../../sample/sample.down.sql");

pub async fn load_sample(conn: &mut PgConnection) {
    #[cfg(feature = "dev")]
    return;

    #[cfg(feature = "local")]
    if let Err(e) = conn.execute(LOAD_SAMPLE_SQL).await {
        panic!("ERROR: {e}");
    }

    #[cfg(not(any(feature = "dev", feature = "local")))]
    return;
}

pub async fn unload_sample(conn: &mut PgConnection) {
    #[cfg(feature = "dev")]
    return;

    #[cfg(feature = "local")]
    if let Err(e) = conn.execute(UNLOAD_SAMPLE_SQL).await {
        panic!("ERROR: {e}");
    }

    #[cfg(not(any(feature = "dev", feature = "local")))]
    return;
}

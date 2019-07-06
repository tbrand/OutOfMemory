use crate::config::Config;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::fs::File;

#[derive(Clone)]
pub struct State {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
    pub config: Config,
}

impl State {
    pub fn new(database_url: impl Into<String>, config_path: impl Into<String>) -> State {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();

        let config_path: String = config_path.into();
        let config_file = File::open(&config_path)
            .map_err(|e| {
                error!("error during opening config file={:?}", e);
            })
            .unwrap();

        let config = serde_yaml::from_reader(config_file)
            .map_err(|e| {
                error!("error during parsing config file={:?}", e);
            })
            .unwrap();

        State { pool, config }
    }
}

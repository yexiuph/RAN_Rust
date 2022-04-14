pub extern crate ini;
pub use ini::Ini;

// Config JSON Object
pub struct DBConfiguration {
    pub database_host: String,
    pub database_port: String,
    pub database_user: String,
    pub database_password: String,
    pub database_name: String,
}

pub struct UpdaterConfig {
    pub updater_version : String,
}

pub fn app_config() -> UpdaterConfig {

    let config = Ini::load_from_file("config.ini").unwrap();

    let version = config.section(Some("Updater")).unwrap().get("VERSION").unwrap();

    UpdaterConfig {
        updater_version: version.to_string(),
    }
}


pub fn ini_loader() -> DBConfiguration {

    let config = Ini::load_from_file("config.ini").unwrap();
    let database_config = config.section(Some("Database")).unwrap();
    let database_host = database_config.get("DB_HOST").unwrap();
    let database_port = database_config.get("DB_PORT").unwrap();
    let database_user = database_config.get("DB_USER").unwrap();
    let database_password = database_config.get("DB_PASS").unwrap();
    let database_name = database_config.get("DB_NAME").unwrap();

    // Return the Configuration
    DBConfiguration {
        database_host: database_host.to_string(),
        database_port: database_port.to_string(),
        database_user: database_user.to_string(),
        database_password: database_password.to_string(),
        database_name: database_name.to_string(),
    }
}
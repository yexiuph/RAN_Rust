mod config;

fn main() {
   let config_data = config::app_config();
   let _database_config = config::ini_loader();

    println!("Updater Version : {}", config_data.updater_version);

}
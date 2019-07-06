pub fn port<'a, 'b>() -> clap::Arg<'a, 'b> {
    clap::Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("PORT")
        .required(true)
        .takes_value(true)
        .default_value("8081")
        .help("Binding port.")
}

pub fn host<'a, 'b>() -> clap::Arg<'a, 'b> {
    clap::Arg::with_name("host")
        .short("H")
        .long("host")
        .value_name("HOST")
        .required(true)
        .takes_value(true)
        .default_value("0.0.0.0")
        .help("Binding host.")
}

pub fn database_url<'a, 'b>() -> clap::Arg<'a, 'b> {
    clap::Arg::with_name("database_url")
        .short("d")
        .long("database_url")
        .value_name("HOST")
        .required(true)
        .takes_value(true)
        .help("URL of MySQL to be connected. (Including database.)")
        .env("DATABASE_URL")
}

pub fn config_path<'a, 'b>() -> clap::Arg<'a, 'b> {
    clap::Arg::with_name("config_path")
        .short("c")
        .long("config")
        .value_name("CONFIG_PATH")
        .required(true)
        .takes_value(true)
        .default_value("config.yaml")
        .help("Path to the config file.")
        .env("CONFIG_PATH")
}

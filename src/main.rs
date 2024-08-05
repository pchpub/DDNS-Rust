// cSpell:ignore ddns, chrono
use ddns_rust::mods::{config::spawn::init_config, handle::spawn_tasks, statics::CONFIG};
use log::{error, trace};
use std::env;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "./config.toml"
    };

    // initialization config
    match init_config(&path).await {
        Err(err) => {
            println!("Error: {}", err);
            panic!("Error: failed to init config in main()");
        }
        _ => (),
    };
    // initialization config end

    // initialization logger
    {
        let log_level = CONFIG.lock().await.log_level.to_string();

        println!("log_level: {}", log_level);
        println!(
            "log_level2: {}",
            log_level
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Info)
        );

        use env_logger::Builder;
        use log::LevelFilter;
        use std::io::Write;
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} [{}] - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .filter(
                None,
                log_level
                    .parse::<LevelFilter>()
                    .unwrap_or(LevelFilter::Info),
            )
            .parse_default_env()
            .init();
    }
    // initialization logger end

    // initialization plugins
    // todo!("initialization plugins")
    // initialization plugins end

    // initialization tasks
    trace!("initialization tasks");
    if let Err(value) = spawn_tasks().await {
        error!("initialization tasks failed: {}", value);
        panic!("Error: failed to spawn tasks in main() reason: {}", value);
    }
    // initialization tasks end

    tokio::signal::ctrl_c().await.unwrap();
}

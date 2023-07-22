use lazy_static::lazy_static;
use std::path::PathBuf;
use dotenv::dotenv;
use std::sync::Once;
use chrono;
use std::str::FromStr;

pub mod argv;

/// Initialize logger. 
/// WARN: Must be called before any logging is done.
fn init_logger() {
    std::fs::create_dir_all(&*LOG_DIR).expect("Failed to create log directory");

    let file_out = fern::log_file(
        (&*LOG_DIR).join("output.log")
    ).expect("Failed to open log file");

    // Parse the log level from LOGGER_MODE
    let log_level = match log::LevelFilter::from_str(LOGGER_MODE.as_str()) {
        Ok(level) => level,
        Err(_) => {
            println!("Invalid LOGGER_MODE value. Defaulting to 'info'.");
            log::LevelFilter::Info
        },
    };

    let logger_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}][{} {}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                chrono::offset::Utc::now().format("%Z"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(file_out)
        .chain(fern::Output::call(|record| {
            println!("{}", record.args());
        }))
        .apply();

    if let Err(e) = logger_config {
        panic!("Failed to initialize logger: {}", e);
    }

    log::info!(" 🚀 starting program...");
}

static INIT: Once = Once::new();

/// Initialize things that need to be initialized only once. 
/// Call this function before doing anything else.
pub fn init() {
    INIT.call_once(|| {
        // initialization code here
        print!("Loading .env file... ");
        dotenv().expect("Failed to load .env file");
        print!("Initializing logger... ");
        init_logger();
    });
}

// Static variables are initialized only once, when they are first used.
lazy_static! {
    pub static ref ARGV: argv::Argv = argv::get_program_args();

    // config
    pub static ref LOGGER_MODE: String = {
        let logger_mode = std::env::var("LOGGER_MODE");
        match logger_mode {
            Ok(mode) => mode,
            Err(_) => {
                println!("LOGGER_MODE environment variable not set. Defaulting to 'info'.");
                return "info".to_string();
            },
        }
    };

    // constants
    pub static ref MAX_NB_OF_LOOP_ITERATIONS: u32 = {
        let max_nb_of_loop_iterations = std::env::var("MAX_NB_OF_LOOP_ITERATIONS");
        match max_nb_of_loop_iterations {
            Ok(max_nb_of_loop_iterations) => {
                match max_nb_of_loop_iterations.parse::<u32>() {
                    Ok(max_nb_of_loop_iterations) => max_nb_of_loop_iterations,
                    Err(_) => {
                        println!("MAX_NB_OF_LOOP_ITERATIONS environment variable is not a valid u32. Defaulting to 1000.");
                        return 1000;
                    },
                }
            },
            Err(_) => {
                println!("MAX_NB_OF_LOOP_ITERATIONS environment variable not set. Defaulting to 1000.");
                return 1000;
            },
        }
    };

    // paths
    // NOTE: remember to test path existence
    pub static ref LOG_DIR: PathBuf = {
        let env_log_dir: String = std::env::var("LOG_DIR")
            .expect("LOG_DIR environment variable must be set in .env");
        let path: PathBuf = PathBuf::from(&env_log_dir);
        check_path(&path);
        path
    };

    // default directory for input files
    pub static ref DEFAULT_INPUT_DIR_PATH: PathBuf = {
        let env_default_input_dir: String = std::env::var("DEFAULT_INPUT_DIR_PATH")
            .expect("DEFAULT_INPUT_DIR_PATH environment variable must be set in .env");
        let path: PathBuf = PathBuf::from(&env_default_input_dir);
        check_path(&path);
        path
    };

}

/// Check if a path exists. If not, panic.
fn check_path(path: &PathBuf) {
    if !path.exists() {
        panic!("Path {:?} does not exist", path);
    }
}
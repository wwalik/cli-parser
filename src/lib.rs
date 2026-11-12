pub mod cli {
    pub mod app;
    pub use app::App;

    pub mod config;
    pub use config::AppConfig;
    pub use config::FlagEntry;

    pub mod modcmd;
    pub use modcmd::Command;
    pub use modcmd::Module;
}

use cosmic_ext_applet_netspeed::config::CONFIG_VERSION;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    tracing::info!("Starting time applet with version {CONFIG_VERSION}");

    cosmic_ext_applet_netspeed::run()
}

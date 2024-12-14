pub mod config;
mod window;

use window::Window;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Window>(())
}

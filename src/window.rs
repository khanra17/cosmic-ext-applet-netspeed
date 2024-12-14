use cosmic::{
    app,
    applet::cosmic_panel_config::PanelAnchor,
    iced::{
        widget::{row, text},
        Alignment, Subscription,
    },
    widget::{autosize, button},
    Element,
};

pub struct Window {
    core: cosmic::app::Core,
    rx_speed: f64, // Download speed in MB/s
    tx_speed: f64, // Upload speed in MB/s
    last_rx: u64,  // Last received bytes
    last_tx: u64,  // Last transmitted bytes
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl Window {
    fn format_speed(speed: f64) -> String {
        if speed >= 1.0 {
            format!("{:.1} MB/s", speed)
        } else {
            format!("{:.1} KB/s", speed * 1024.0)
        }
    }

    fn update_speeds(&mut self) {
        // Read current network stats
        if let Ok(stats) = std::fs::read_to_string("/proc/net/dev") {
            let mut rx_bytes = 0;
            let mut tx_bytes = 0;

            // Parse network interface statistics
            for line in stats.lines().skip(2) {
                // Skip header lines
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 9 && !parts[0].starts_with("lo:") {
                    rx_bytes += parts[1].parse::<u64>().unwrap_or(0);
                    tx_bytes += parts[9].parse::<u64>().unwrap_or(0);
                }
            }

            // Calculate speeds
            let rx_diff = rx_bytes.saturating_sub(self.last_rx);
            let tx_diff = tx_bytes.saturating_sub(self.last_tx);

            self.rx_speed = rx_diff as f64 / (1024.0 * 1024.0); // Convert to MB/s
            self.tx_speed = tx_diff as f64 / (1024.0 * 1024.0);

            self.last_rx = rx_bytes;
            self.last_tx = tx_bytes;
        }
    }
}

impl cosmic::Application for Window {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    const APP_ID: &'static str = "io.github.khanra17.cosmic-ext-applet-netspeed";

    fn init(
        core: app::Core,
        _flags: Self::Flags,
    ) -> (Self, cosmic::iced::Task<app::Message<Self::Message>>) {
        (
            Self {
                core,
                rx_speed: 0.0,
                tx_speed: 0.0,
                last_rx: 0,
                last_tx: 0,
            },
            cosmic::iced::Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> cosmic::iced::Task<app::Message<Self::Message>> {
        if let Message::Tick = message {
            self.update_speeds();
        }
        cosmic::iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        let horizontal = matches!(
            self.core.applet.anchor,
            PanelAnchor::Top | PanelAnchor::Bottom
        );

        let content = if horizontal {
            row![
                text(format!("↓{}", Self::format_speed(self.rx_speed))),
                text(format!("↑{}", Self::format_speed(self.tx_speed))),
            ]
            .spacing(5)
            .align_y(Alignment::Center)
        } else {
            row![
                text(format!("↓{}", Self::format_speed(self.rx_speed))),
                text(format!("↑{}", Self::format_speed(self.tx_speed))),
            ]
            .spacing(5)
            .align_y(Alignment::Center)
        };

        let button = button::custom(content)
            .padding([
                self.core.applet.suggested_padding(horizontal),
                self.core.applet.suggested_padding(!horizontal),
            ])
            .class(cosmic::theme::Button::AppletIcon);

        autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }

    fn on_close_requested(&self, _id: cosmic::iced::window::Id) -> Option<Message> {
        None
    }
}

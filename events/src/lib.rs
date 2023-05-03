use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = self.color;
        let colored_content = match color {
            (255, 2, 22) => self.content.red(),
            (200, 200, 3) => self.content.yellow(),
            (0, 255, 0) => self.content.green(),
            _ => self.content.normal(),
        };
        let position_str = match self.position {
            Position::Top => "Top",
            Position::Bottom => "Bottom",
            Position::Center => "Center",
        };
        write!(
            f,
            "({}, {}, {})",
            position_str,
            self.size,
            colored_content
        )
    }
}

use Event::*;

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(duration) => {
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
                let content = format!(
                    "You have {:02}H:{:02}M:{:02}S left before the registration ends",
                    hours, minutes, seconds
                );
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content,
                }
            }
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
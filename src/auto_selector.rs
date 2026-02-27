use std::time::Duration;
use vexide::{
    color::Color,
    display::{Font, FontFamily, FontSize, Rect, Text, TouchState},
    prelude::*,
};

pub async fn select_auto(display: &mut Display, _controller: &Controller) -> u8 {
    let mut selected = 1;
    let max_auto = 8;
    let confirm_rect = Rect::new([50, 140], [430, 240]);

    loop {
        // Clear display
        display.fill(&Rect::new([0, 0], [480, 272]), Color::new(0, 0, 0));

        // Draw current selection - hardcode the names since c! won't work with variables
        let selection_text = match selected {
            1 => c"l4bcombo m",
            2 => c"dist sensor test",
            3 => c"r7bpid",
            4 => c"l7bpid",
            5 => c"shittysawp",
            6 => c"testr7bseek",
            7 => c"",
            8 => c"",
            _ => c"Unknown",
        };
        let selection = Text::new(
            selection_text,
            Font::new(FontSize::new(1, 1), FontFamily::Proportional),
            [0, 0],
        );
        display.draw_text(&selection, Color::new(255, 255, 255), None);

        // Draw confirm button (filled)
        display.fill(&confirm_rect, Color::new(50, 50, 150));
        display.stroke(&confirm_rect, Color::new(100, 100, 255));
        let confirm_text = Text::new(c"CONFIRM", Font::new(FontSize::new(2, 2), FontFamily::Proportional), [170, 175]);
        display.draw_text(&confirm_text, Color::new(100, 100, 255), None);

        // Check for touch
        let touch = display.touch_status();
        if touch.state != TouchState::Released {
            // Check confirm button first
            if touch.point.x >= confirm_rect.top_left.x
                && touch.point.x <= confirm_rect.bottom_right.x
                && touch.point.y >= confirm_rect.top_left.y
                && touch.point.y <= confirm_rect.bottom_right.y
            {
                break;
            } else if touch.point.x < 240 && selected > 1 {
                selected -= 1;
                sleep(Duration::from_millis(200)).await;
            } else if touch.point.x >= 240 && selected < max_auto {
                selected += 1;
                sleep(Duration::from_millis(200)).await;
            }
        }

        sleep(Duration::from_millis(50)).await;
    }

    selected
}

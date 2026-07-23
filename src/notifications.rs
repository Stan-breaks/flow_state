use chrono::Local;

use notify_rust::Notification;
use cron::ScheduleIterator;
use std::iter::Peekable;

#[derive(Clone)]
pub struct NotificationData {
    pub done: usize,
    pub total: usize,
    pub low_threshold: usize,
    pub high_threshold: usize,
}

impl Default for NotificationData {
    fn default() -> Self {
        NotificationData {
            done: 0,
            total: 0,
            low_threshold: 20,
            high_threshold: 80,
        }
    }
}

impl NotificationData {
    pub fn get_percent(&self) -> f32 {
        if self.total == 0 {
            return 100.0;
        }
        return 100.0 * (self.done as f32) / (self.total as f32);
    }

    pub fn get_notification_text(&self) -> String {
        let progress = self.get_percent();
        if progress <= self.low_threshold as f32 {
            return String::from("I know you're busy, but make sure to check your habit tracker today!");
        } else if progress >= self.high_threshold as f32 {
            return String::from("You've done nearly all your tasks today, well done!");
        } else {
            return String::from("");
        }
    }
}

pub fn check_notification_trigger(count: NotificationData, iter: &mut Peekable<ScheduleIterator<'_, Local>>) {
    let datetime = iter.peek();
    let now = Local::now();
    match datetime {
        Some(x) => if *x <= now {
            send_notification(count);
            iter.next();
        }
        None    => {},
    }
}

pub fn send_notification(count: NotificationData) {
    let s = count.get_notification_text();
    if !s.is_empty() {
        let mut notification = Notification::new();
        notification.summary("flow_state reminder").body(&s[..]).timeout(0);
        // urgency() is a Linux/D-Bus-only builder method — macOS's
        // notification backend doesn't expose it.
        #[cfg(not(target_os = "macos"))]
        notification.urgency(notify_rust::Urgency::Normal);
        let _ = notification.show();
    }
}



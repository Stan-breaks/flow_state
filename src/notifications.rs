use chrono::Local;

use notify_rust::Notification;
use cron::ScheduleIterator;
use std::iter::Peekable;

use crate::app::NotificationData;

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
        let _ = Notification::new().summary("flow_state reminder")
            .body(&s[..])
            .timeout(0)
            .urgency(notify_rust::Urgency::Normal)
            .show();
    }
}



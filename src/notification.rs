use notify_rust::Notification;

fn send_notification(title: &str, body: &str) {
    Notification::new()
        .summary(title)
        .body(body)
        .show()
        .expect("cannot happen");
}

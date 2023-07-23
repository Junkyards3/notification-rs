use crate::app::NotificationTask;
use crate::db::NotificationContent;
use chrono::OutOfRangeError;
use notify_rust::Notification;

pub fn prepare_notification_sending_task(
    notification: NotificationContent,
) -> Result<NotificationTask, OutOfRangeError> {
    let duration = (notification.date - chrono::Local::now().naive_local()).to_std()?;
    let title = notification.title.clone();
    let body = notification.body.clone();
    let handle = tokio::spawn(async move {
        tokio::time::sleep(duration).await;
        Notification::new()
            .summary(&title)
            .body(&body)
            .show()
            .expect("cannot happen");
    });
    Ok(NotificationTask {
        content: notification,
        handle,
    })
}

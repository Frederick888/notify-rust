use winrt_toast::{Image, Toast, ToastDuration, ToastManager};

pub use crate::{error::*, notification::Notification, timeout::Timeout};

const POWERSHELL_APP_ID: &str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\
                                             \\WindowsPowerShell\\v1.0\\powershell.exe";

pub(crate) fn show_notification(notification: &Notification) -> Result<()> {
    // let sound = match &notification.sound_name {
    //     Some(chosen_sound_name) => winrt_notification::Sound::from_str(chosen_sound_name).ok(),
    //     None => None,
    // };

    let duration = match notification.timeout {
        Timeout::Default => ToastDuration::Short,
        Timeout::Never => ToastDuration::Long,
        Timeout::Milliseconds(t) => {
            if t >= 25000 {
                ToastDuration::Long
            } else {
                ToastDuration::Short
            }
        }
    };

    let manager = ToastManager::new(POWERSHELL_APP_ID);
    let mut toast = Toast::new();
    toast
        .text1(notification.subtitle.as_ref().map(AsRef::as_ref).unwrap_or(""))
        .text2(&notification.body)
        .duration(duration);
    if let Some(image_path) = &notification.path_to_image {
        let image = Image::new_local(&image_path).map_err(|e| Error::from(ErrorKind::Msg(format!("{:?}", e))))?;
        toast.image(1, image);
    }

    manager
        .show(&toast)
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{:?}", e))))
}

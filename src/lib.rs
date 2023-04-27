//! This crate provides a trait [`DialogExpect`], with a function that unwraps [`Option<T>`] and [`Result<T, E>`] or shows a dialog box and panics.

use native_dialog::MessageType;

/// A trait with 1 function, `dialog_expect`, which is implemented for [`std::option::Option<T>`] and [`std::result::Result<T, E>`].
pub trait DialogExpect<T> {
    /// Takes `self` and returns the contained value, or shows an error message in a dialog box.
    /// #### Errors
    /// This function can fail (the program will panic) if there is no way to spawn a dialog box. It just made no sense for it to return a [`Result<T, E>`].
    fn dialog_expect<S: AsRef<str>>(self, title: S, msg: S) -> T;
}

impl<T> DialogExpect<T> for std::option::Option<T> {
    fn dialog_expect<S: AsRef<str>>(self, title: S, msg: S) -> T {
        match self {
            Some(v) => v,
            None => {
                let msg = msg.as_ref();

                native_dialog::MessageDialog::new()
                    .set_title(title.as_ref())
                    .set_text(msg)
                    .set_type(MessageType::Error)
                    .show_alert()
                    .unwrap();
                panic!("{}", msg);
            }
        }
    }
}

impl<T, E> DialogExpect<T> for std::result::Result<T, E> {
    fn dialog_expect<S: AsRef<str>>(self, title: S, msg: S) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                let msg = msg.as_ref();

                native_dialog::MessageDialog::new()
                    .set_title(title.as_ref())
                    .set_text(msg)
                    .set_type(MessageType::Error)
                    .show_alert()
                    .unwrap();
                panic!("{}", msg);
            }
        }
    }
}

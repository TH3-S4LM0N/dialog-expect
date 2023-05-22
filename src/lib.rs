//! This crate provides a trait [`DialogExpect`], with a function that unwraps [`Option<T>`] and [`Result<T, E>`] or shows a dialog box and panics. There is also an alternative to the `panic!()` macro that is the same thing, but it shows a dialog box first.

use native_dialog::{MessageDialog, MessageType};

/// A trait with 1 function, `dialog_expect`, which is implemented for [`std::option::Option<T>`] and [`std::result::Result<T, E>`].
pub trait DialogExpect<T> {
    /// Takes `self` and returns the contained value, or shows an error message in a dialog box.
    /// ### Failures
    /// This function can fail (nothing will happen, but the program will still panic with the message to stdout) if there is no way to show a dialog box (most likely unsupported platform).
    fn dialog_expect(self, title: &str, msg: &str) -> T;
}

impl<T> DialogExpect<T> for std::option::Option<T> {
    fn dialog_expect(self, title: &str, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                dialog_panic_internal(title, msg);
            }
        }
    }
}

impl<T, E> DialogExpect<T> for std::result::Result<T, E> {
    fn dialog_expect(self, title: &str, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                dialog_panic_internal(title, msg);
            }
        }
    }
}

#[allow(unused_must_use)]
#[allow(dead_code)]
pub(crate) fn dialog_panic_internal(title: &str, msg: &str) -> ! {
    MessageDialog::new()
        .set_title(title)
        .set_text(msg)
        .set_type(MessageType::Error)
        .show_alert();

    panic!("{msg}");
}

/// An alternative to `panic!()` showing a dialog box. This is a function because I honestly couldn't figure out how to have a macro taking multiple args.
pub fn dialog_panic(title: &str, msg: &str) -> ! {
    dialog_panic_internal(title, msg);
}

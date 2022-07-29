use native_ui_gnome::{
    NativeApplication,
    abstracts
};
// goal is to only replace
// native_ui_gnome::
// crate for changing the platform to for example
// plasma (qt), windows, or macOS

fn main() {
    let abst_app = abstracts::Application::builder()
        .title("native-ui GNOME test")
        .add_window(
            abstracts::Window::builder()
                .title("test title")
                .build()
        )
        .add_window(
            abstracts::Window::builder()
                .title("Test Title")
                .build()
        )
        .build();
    let mut app = NativeApplication::new(abst_app);
    app.run();
}

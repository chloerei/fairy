use gpui_kit::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child("Hello, World!")
    }
}

fn open_hello_window(cx: &mut App) {
    let options = WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Hello, World!".into()),
            ..Default::default()
        }),
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(480.), px(320.)),
            cx,
        ))),
        ..Default::default()
    };

    gpui_kit::open_window(options, cx, |_, cx| cx.new(|_| HelloWorld))
        .expect("failed to open window");
}

fn main() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    // Recreate the window when reopened from the Dock with none open.
    app.on_reopen(|cx| {
        if cx.windows().is_empty() {
            open_hello_window(cx);
        }
    });

    app.run(|cx| {
        gpui_kit::init(cx);
        open_hello_window(cx);
    });
}

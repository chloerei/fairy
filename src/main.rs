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

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

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
        });
}

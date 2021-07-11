use web_view::*;

fn main() {
    web_view::builder()
        .title("Windy")
        .content(Content::Url(
            "https://www.windy.com/-Weather-radar-radar?radar,49.785,14.903,7",
        ))
        .size(800, 300)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

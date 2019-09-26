fn main() {
    let wv1 = webview::WebView::default();

    let wv2 = webview::WebViewBuilder::new()
        .title_bar(false)
        .width(200.)
        .height(200.)
        .x(100.)
        .y(100.)
        .build();

    let wv3 = webview::WebViewBuilder::new()
        .title("At least a title!")
        .width(400.)
        .height(400.)
        .x(300.)
        .y(300.)
        .miniaturizable(false)
        .resizable(false)
        .build();

    webview::run(&[wv1, wv2, wv3]);
}

use spider_webview::{DataURL, WindowBuilder};

const html: &str = r#"
    <html>
        <head>
            <script>
                window.postMessage("test");
                // window.addEventListener("message", (event) => {
                //     document.write(event.data + '<br>');
                //     event.source.postMessage(event.data)
                // }, false);
            </script>
        </head>
        <body>
            <h1>Hello world!</h1>
        </body>
    </html>
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window =
        WindowBuilder::new().open(&DataURL("text/html;charset=US-ASCII", html).to_string())?;

    window.run();

    // window.focus();
    // window.add_event_listener("message", |event| {
    //     println!("{}", event.data);
    // });

    // 5..0.for_each(|i| {
    //     window.post_message(&format!("leaving page in {}", i));
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // });

    // window.location = "https://news.ycombinator.com";
    // std::thread::sleep(std::time::Duration::from_secs(5));

    // window.close()?;

    Ok(())
}

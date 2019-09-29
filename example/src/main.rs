use spider_webview::{DataURL, Event, WindowConfiguration};

const HTML: &str = r#"
    <html>
        <head>
            <script>
                window.addEventListener('DOMContentLoaded', () => window.postMessage({ event: 'DOMContentLoaded' }));
                document.addEventListener('click', () => window.postMessage({ event: 'click' }));
            </script>
        </head>
        <body>
            <h1>Hello world!</h1>
        </body>
    </html>
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = WindowConfiguration::new();
    let window = config.open(&DataURL("text/html;charset=US-ASCII", HTML).to_string())?;

    // window.focus();
    window.add_event_listener(
        "message",
        Box::new(|event: &Event| println!("{}", event.data)),
    );

    window.run();

    // 5..0.for_each(|i| {
    //     window.post_message(&format!("leaving page in {}", i));
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // });

    // window.location = "https://news.ycombinator.com";
    // std::thread::sleep(std::time::Duration::from_secs(5));

    // window.close()?;

    Ok(())
}

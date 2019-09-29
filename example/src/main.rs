use spider_webview::{step, DataURL, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new()
        .top(200.)
        .left(200.)
        .open(DataURL("text/html;charset=US-ASCII", r#"
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
        "#))?;

    window.add_event_listener("message", &|event| println!("{:?}", event.data));

    for i in (1..=5).rev() {
        window.post_message(&format!("leaving page in {}", i))?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    loop {
        step();
    }

    // window.location = "https://news.ycombinator.com";
    // std::thread::sleep(std::time::Duration::from_secs(5));

    // window.close()?;

    Ok(())
}

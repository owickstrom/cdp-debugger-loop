use std::error::Error;
use std::sync::Arc;

use chromiumoxide::cdp::js_protocol::debugger;
use chromiumoxide::cdp::js_protocol::runtime;
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let browser_config = BrowserConfig::builder();
    let (browser, mut handler) = Browser::launch(browser_config.with_head().build()?).await?;

    let _handle = async_std::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });


    let page = Arc::new(browser.new_page("https://wikipedia.org").await?);

    page.execute(runtime::EnableParams{}).await?;
    page.execute(debugger::EnableParams::builder().build()).await?;

    for i in 1..100 {
        let mut paused_events = page.event_listener::<debugger::EventPaused>().await?;
        let pause_event = paused_events.next();
        page.execute(debugger::PauseParams {}).await?;
        let _ = page.evaluate("true");
        pause_event.await;

        println!("Taking screenshot {i}");
        page.screenshot(ScreenshotParams::builder()
                        // .full_page(false)
                        // .quality(10)
                        // .clip(Viewport { x: 0.0, y: 0.0, width: 100.0, height: 100.0, scale: 1.0})
                        // .from_surface(true)
                        // .omit_background(true)
                        .build()).await?;

        let mut resumed_events = page.event_listener::<debugger::EventResumed>().await?;
        page.execute(debugger::ResumeParams::builder().build())
            .await?;
        resumed_events.next().await;
    }

    Ok(())
}

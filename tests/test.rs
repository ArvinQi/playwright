#[macro_use]
extern crate serde;

mod browser;
mod browser_context;
mod browser_type;
mod devices;
mod page;
mod selectors;

#[cfg(feature = "rt-async-std")]
use async_std::task::spawn;
#[cfg(feature = "rt-actix")]
use tokio::task::spawn;
#[cfg(feature = "rt-tokio")]
use tokio::task::spawn;

use playwright::Playwright;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    Webkit,
    Firefox,
    Chromium
}

playwright::runtime_test!(chromium_page, page(Which::Chromium).await);
playwright::runtime_test!(firefox_page, page(Which::Firefox).await);
// playwright::runtime_test!(webkit_page, page(Which::Webkit).await);

playwright::runtime_test!(chromium_selectors, selectors(Which::Chromium).await);
playwright::runtime_test!(firefox_selectors, selectors(Which::Firefox).await);
// playwright::runtime_test!(webkit_selectors, selectors(Which::Webkit).await);

playwright::runtime_test!(chromium_devices, devices(Which::Chromium).await);
playwright::runtime_test!(firefox_devices, devices(Which::Chromium).await);

async fn page(which: Which) {
    let port = free_local_port().unwrap();
    start_test_server(port).await;
    let playwright = playwright_with_driver().await;
    install_browser(&playwright, which);
    let browser_type = browser_type::all(&playwright, which).await;
    let browser = browser::all(&browser_type, which).await;
    let persistent = browser_context::persistent(&browser_type, port, which).await;
    let browser_context = browser_context::all(&browser, &persistent, which).await;
    page::all(&browser_context, port, which).await;
}

async fn selectors(which: Which) {
    let playwright = playwright_with_driver().await;
    selectors::all(&playwright, which).await;
}

async fn devices(which: Which) {
    let port = free_local_port().unwrap();
    start_test_server(port).await;
    let playwright = playwright_with_driver().await;
    devices::all(&playwright, port, which).await;
}

fn install_browser(p: &Playwright, which: Which) {
    match which {
        Which::Webkit => p.install_webkit(),
        Which::Firefox => p.install_firefox(),
        Which::Chromium => p.install_chromium()
    }
    .unwrap();
}

async fn playwright_with_driver() -> Playwright {
    use playwright::Driver;
    let driver = Driver::new(Driver::default_dest());
    let mut playwright = Playwright::with_driver(driver).await.unwrap();
    let _ = playwright.driver();
    playwright
}

#[cfg(any(feature = "rt-tokio", feature = "rt-actix"))]
async fn start_test_server(port: u16) {
    use warp::Filter;
    let route = warp::path("static").and(warp::fs::dir("tests/server"));
    spawn(async move {
        warp::serve(route).run(([127, 0, 0, 1], port)).await;
    });
}

#[cfg(feature = "rt-async-std")]
async fn start_test_server(port: u16) {
    use tide::Server;
    let mut app = Server::new();
    app.at("/static").serve_dir("tests/server/").unwrap();
    spawn(async move {
        app.listen(format!("127.0.0.1:{}", port)).await.unwrap();
    });
}

fn free_local_port() -> Option<u16> {
    let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
    std::net::TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

fn url_static(port: u16, path: &str) -> String {
    format!("http://localhost:{}/static{}", port, path)
}

//    let h = page.eval_handle("() => location.href").await.unwrap();
//    let s: String = page
//        .evaluate("([s]) => s + location.href", Some(vec![h]))
//        .await
//        .unwrap();
//    assert_eq!(s, "https://example.com/https://example.com/");
//    let s: DateTime = page
//        .evaluate("d => d", Some(DateTime::from(chrono::Utc::now())))
//        .await
//        .unwrap();
//    println!("{:?}", s);
//    let (next_page, _) = tokio::join!(
//        context.expect_event(browser_context::EventType::Page),
//        page.click_builder("a")
//            .modifiers(vec![KeyboardModifier::Control])
//            .click()
//    );
//    let _next_page = match next_page.unwrap() {
//        browser_context::Event::Page(p) => p,
//        _ => unreachable!()
//    };
//    //// let _ = p.main_frame().query_selector_all("a").await.unwrap();
//    //// let _ = p.main_frame().title().await.unwrap();
//    // let mut a = p.query_selector("a").await.unwrap().unwrap();
//    // let _href = a.get_attribute("href").await.unwrap();

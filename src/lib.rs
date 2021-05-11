use mobile_entry_point::mobile_entry_point;
use std::collections::HashMap;
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, WindowEvent, StartCause},
        event_loop::{ControlFlow, EventLoop},
        platform::ios::{WindowBuilderExtIOS, WindowExtIOS, ScreenEdge},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

#[cfg(target_os = "macos")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("wry"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new().init().unwrap();
}

#[mobile_entry_point]
fn main() {
    init_logging();
    let event_loop = EventLoop::new();

    let mut weviews = HashMap::new();


    let window = WindowBuilder::new()
        .with_preferred_screen_edges_deferring_system_gestures(ScreenEdge::all())
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url("https://tauri.studio")
        .unwrap()
        .build()
        .unwrap();

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Init");

                let window = WindowBuilder::new()
                .build(&event_loop)
                .unwrap();
                let window_id = window.id();
    
                let weview = WebViewBuilder::new(window)
                    .unwrap()
                    .with_url("https://tauri.studio")
                    .unwrap()
                    .build()
                    .unwrap();
                weviews.insert(window_id, weview);
            },
            Event::Resumed => {
                println!("applicationDidBecomeActive");
                
            }
            Event::Suspended => {
                println!("applicationWillResignActive");
            }
            Event::LoopDestroyed => {
                println!("applicationWillTerminate");
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Touch(touch)
            } => {
                println!("touch on {:?} {:?}", window_id, touch);
            }
            _ => {}
        }
    });
}

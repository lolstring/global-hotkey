use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

use iced::futures::{SinkExt, Stream};
use iced::stream::channel;
use iced::widget::{column, container, text};
use iced::{application, Element, Subscription, Task, Theme, Length};
use std::collections::HashMap;

fn main() -> iced::Result {
    application("Modifiers Example", update, view)
        .subscription(subscription)
        .theme(|_| Theme::Dark)
        .run_with(new)
}

struct Example {
    last_event: String,
    hotkey_descriptions: HashMap<u32, String>,
    // store the global manager otherwise it will be dropped and events will not be emitted
    _manager: GlobalHotKeyManager,
}

#[derive(Debug, Clone)]
enum Message {
    EventReceived(GlobalHotKeyEvent),
}

fn new() -> (Example, Task<Message>) {
    let manager = GlobalHotKeyManager::new().unwrap();
    let mut hotkey_descriptions = HashMap::new();

    // Define hotkeys to register
    let hotkeys = vec![
        // Individual Modifier Keys (as triggers)
        ("Control Left", HotKey::new(None, Code::ControlLeft)),
        ("Control Right", HotKey::new(None, Code::ControlRight)),
        // ("Shift Left", HotKey::new(None, Code::ShiftLeft)),
        ("Shift Right", HotKey::new(None, Code::ShiftRight)),
        ("Alt Left", HotKey::new(None, Code::AltLeft)),
        ("Alt Right", HotKey::new(None, Code::AltRight)),
        // ("Meta Left", HotKey::new(None, Code::MetaLeft)),
        ("Meta Right", HotKey::new(None, Code::MetaRight)),
        ("Fn", HotKey::new(None, Code::Fn)),

        // Simple Combinations
        ("Control + A", HotKey::new(Some(Modifiers::CONTROL), Code::KeyA)),
        ("Shift + B", HotKey::new(Some(Modifiers::SHIFT), Code::KeyB)),
        ("Alt + C", HotKey::new(Some(Modifiers::ALT), Code::KeyC)),
        ("Meta + D", HotKey::new(Some(Modifiers::META), Code::KeyD)),
        
        // Specific Modifier Combinations
        ("ControlRight + ArrowUp", HotKey::new(Some(Modifiers::CONTROL_RIGHT), Code::ArrowUp)),
        ("ShiftLeft + Space", HotKey::new(Some(Modifiers::SHIFT_LEFT), Code::Space)),
        
        // Complex Combinations
        ("Control + Shift + Enter", HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Enter)),
        ("String Key + MetaLeft", "MetaLeft".parse().unwrap()),
        ("String + ShiftLeft", "ShiftLeft".parse().unwrap()),
        // ("String + Function", "Fn".parse().unwrap()),
    ];

    println!("Registering hotkeys...");
    for (desc, hk) in hotkeys {
        match manager.register(hk) {
            Ok(_) => {
                println!("Registered: {}", desc);
                hotkey_descriptions.insert(hk.id(), desc.to_string());
            }
            Err(e) => {
                println!("Failed to register {}: {:?}", desc, e);
            }
        }
    }

    (
        Example {
            last_event: "Press any registered modifier or hotkey...".to_string(),
            hotkey_descriptions,
            _manager: manager,
        },
        Task::none(),
    )
}

fn update(state: &mut Example, msg: Message) -> Task<Message> {
    match msg {
        Message::EventReceived(event) => {
            let state_str = match event.state {
                HotKeyState::Pressed => "Pressed",
                HotKeyState::Released => "Released",
            };
            
            if let Some(desc) = state.hotkey_descriptions.get(&event.id) {
                state.last_event = format!("{} ({})", desc, state_str);
            } else {
                 state.last_event = format!("Unknown ID: {} ({})", event.id, state_str);
            }
            Task::none()
        }
    }
}

fn view(state: &Example) -> Element<'_, Message> {
    container(
        column![
            text("Global Hotkey Modifiers Example").size(24),
            text("Try pressing these keys:"),
            text(" - Single Modifiers: Control(L/R), Shift(L/R), Alt(L/R), Meta(L/R)"),
            text(" - Combinations: Ctrl+A, Shift+B, Alt+C, Meta+D"),
            text(" - Specific: RightControl+Up, LeftShift+Space"),
            text(" - Complex: Ctrl+Shift+Enter"),
            text(""), // spacer
            text("Last Event:"),
            text(&state.last_event).size(30).color([0.5, 1.0, 0.5]),
        ]
        .spacing(10)
        .padding(20)
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn subscription(_state: &Example) -> Subscription<Message> {
    Subscription::run(hotkey_sub)
}

fn hotkey_sub() -> impl Stream<Item = Message> {
    channel(100, |mut sender| async move {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(event) = receiver.try_recv() {
                sender.send(Message::EventReceived(event)).await.unwrap();
            }
            async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
}

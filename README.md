global_hotkey lets you register Global HotKeys for Desktop Applications.

## Platforms-supported:

- Windows
- macOS
- Linux (X11 Only)

## Platform-specific notes:

- On Windows a win32 event loop must be running on the thread. It doesn't need to be the main thread but you have to create the global hotkey manager on the same thread as the event loop.
- On macOS, an event loop must be running on the main thread so you also need to create the global hotkey manager on the main thread.

## Modifier Keys as Standalone Hotkeys

You can register individual modifier keys (e.g., `ControlLeft`, `ShiftRight`) as standalone hotkeys:

```rs
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Code}};

let manager = GlobalHotKeyManager::new().unwrap();

// Register left Control key as a standalone hotkey
let hotkey = HotKey::new(None, Code::ControlLeft);
manager.register(hotkey).unwrap();

// Register right Shift key
let hotkey = HotKey::new(None, Code::ShiftRight);
manager.register(hotkey).unwrap();
```

### Left/Right Specific Modifiers

You can also use left/right specific modifiers in combinations:

```rs
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};

let manager = GlobalHotKeyManager::new().unwrap();

// Only triggers when RIGHT Control + A is pressed
let hotkey = HotKey::new(Some(Modifiers::CONTROL_RIGHT), Code::KeyA);
manager.register(hotkey).unwrap();

// Only triggers when LEFT Shift + Space is pressed
let hotkey = HotKey::new(Some(Modifiers::SHIFT_LEFT), Code::Space);
manager.register(hotkey).unwrap();
```

Available specific modifiers: `SHIFT_LEFT`, `SHIFT_RIGHT`, `CONTROL_LEFT`, `CONTROL_RIGHT`, `ALT_LEFT`, `ALT_RIGHT`, `SUPER_LEFT`, `SUPER_RIGHT`, `FN`(Mac Keyboard).

### macOS Permissions

On macOS, registering standalone modifier keys requires **Accessibility permissions**. The app must be granted access in **System Preferences > Security & Privacy > Privacy > Accessibility**.

If accessibility permissions are not granted, the library will attempt to fall back to the standard hotkey registration mechanism, which may have limited functionality for modifier-only hotkeys.

## Example

```rs
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};

// initialize the hotkeys manager
let manager = GlobalHotKeyManager::new().unwrap();

// construct the hotkey
let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);

// register it
manager.register(hotkey);
```

## Parsing Hotkeys from Strings

You can also create hotkeys by parsing strings:

```rs
use global_hotkey::hotkey::HotKey;

let hotkey: HotKey = "Ctrl+Shift+KeyA".parse().unwrap();
let hotkey: HotKey = "CommandOrControl+C".parse().unwrap();
let hotkey: HotKey = "Ctrl+ShiftLeft".parse().unwrap(); // Modifier as main key
```

### String Format

**Format:** `modifier+modifier+...+key`

**Rules:**
- Tokens separated by `+`
- Case insensitive (`ctrl+a` = `CTRL+A`)
- Whitespace around tokens is trimmed
- **Last token is always the main key**
- All tokens before the last must be valid modifiers

**Allowed:**
- Single key: `KeyA`, `ShiftLeft`, `Fn`
- Modifiers + key: `Ctrl+Shift+KeyA`
- Modifier as main key: `Ctrl+ShiftLeft`, `Shift+Ctrl`

**Not Allowed:**
- Empty tokens: `Ctrl++A`
- Non-modifier before last token: `KeyA+Ctrl`
- Unknown key names: `Ctrl+Foo`

### Special Keys

| Category | Keys | Notes |
|----------|------|-------|
| **Fn** | `Fn`, `Function` | macOS only, requires accessibility permissions |
| **Media** | `MediaPlayPause`, `MediaTrackNext`, `MediaTrackPrevious`, `MediaPlay`, `MediaPause`, `MediaStop` | May require event tap on macOS |
| **Volume** | `VolumeUp`, `VolumeDown`, `VolumeMute` | Also: `AudioVolumeUp`, etc. |
| **Standalone Modifiers** | `ShiftLeft`, `ShiftRight`, `ControlLeft`, `ControlRight`, `AltLeft`, `AltRight`, `MetaLeft`, `MetaRight` | Can be used as main key |

### Modifier Aliases

| Modifier | Aliases |
|----------|---------|
| `Alt` | `Option` |
| `Control` | `Ctrl` |
| `Super` | `Command`, `Cmd`, `Meta` |
| `CmdOrCtrl` | `CommandOrControl` — Super on macOS, Control elsewhere |

### Side-Specific Modifiers

- `ShiftLeft`, `ShiftRight`
- `ControlLeft` / `CtrlLeft`, `ControlRight` / `CtrlRight`
- `AltLeft` / `OptionLeft`, `AltRight` / `OptionRight`
- `SuperLeft` / `CommandLeft` / `CmdLeft`, `SuperRight` / `CommandRight` / `CmdRight`

## Processing global hotkey events

You can also listen for the menu events using `GlobalHotKeyEvent::receiver` to get events for the hotkey pressed events.

```rs
use global_hotkey::GlobalHotKeyEvent;

if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
    println!("{:?}", event);
}
```

## License

Apache-2.0/MIT

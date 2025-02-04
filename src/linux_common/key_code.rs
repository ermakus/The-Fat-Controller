use crate::Key;

pub fn to_key_code(key: Key) -> u16 {
    use Key::*;
    use super::ffi::*;
    match key {
        CapsLock => KEY_CAPSLOCK,
        Shift => KEY_LEFTSHIFT,
        Control => KEY_LEFTCTRL,
        Alt => KEY_LEFTALT,
        Meta => KEY_LEFTMETA,
        RightShift => KEY_RIGHTSHIFT,
        RightControl => KEY_RIGHTCTRL,
        RightAlt => KEY_RIGHTALT,
        RightMeta => KEY_RIGHTMETA,
        Fn => KEY_RESERVED,
        ReturnOrEnter => KEY_ENTER,
        Escape => KEY_ESC,
        DeleteOrBackspace => KEY_BACKSPACE,
        ForwardDelete => KEY_DELETE,
        Tab => KEY_TAB,
        Space => KEY_SPACE,
        Minus => KEY_MINUS,
        Equal => KEY_EQUAL,
        LeftBracket => KEY_LEFTBRACE,
        RightBracket => KEY_RIGHTBRACE,
        Backslash => KEY_BACKSLASH,
        Semicolon => KEY_SEMICOLON,
        Quote => KEY_APOSTROPHE,
        Grave => KEY_GRAVE,
        Comma => KEY_COMMA,
        Period => KEY_DOT,
        Slash => KEY_SLASH,
        IntlBackslash => KEY_102ND, // TODO
        Apps => KEY_COMPOSE,        // TODO: ???
        UpArrow => KEY_UP,
        RightArrow => KEY_RIGHT,
        DownArrow => KEY_DOWN,
        LeftArrow => KEY_LEFT,
        PageUp => KEY_PAGEUP,
        PageDown => KEY_PAGEDOWN,
        Home => KEY_HOME,
        End => KEY_END,
        Insert => KEY_INSERT,
        Print => KEY_PRINT,
        PrintScreen => KEY_SYSRQ,   // TODO: ???
        ScrollLock => KEY_SCROLLLOCK,// TODO
        Pause => KEY_PAUSE,         // TODO
        NumLock => KEY_NUMLOCK,
        A => KEY_A,
        B => KEY_B,
        C => KEY_C,
        D => KEY_D,
        E => KEY_E,
        F => KEY_F,
        G => KEY_G,
        H => KEY_H,
        I => KEY_I,
        J => KEY_J,
        K => KEY_K,
        L => KEY_L,
        M => KEY_M,
        N => KEY_N,
        O => KEY_O,
        P => KEY_P,
        Q => KEY_Q,
        R => KEY_R,
        S => KEY_S,
        T => KEY_T,
        U => KEY_U,
        V => KEY_V,
        W => KEY_W,
        X => KEY_X,
        Y => KEY_Y,
        Z => KEY_Z,
        N0 => KEY_0,
        N1 => KEY_1,
        N2 => KEY_2,
        N3 => KEY_3,
        N4 => KEY_4,
        N5 => KEY_5,
        N6 => KEY_6,
        N7 => KEY_7,
        N8 => KEY_8,
        N9 => KEY_9,
        Numpad0 => KEY_KP0,
        Numpad1 => KEY_KP1,
        Numpad2 => KEY_KP2,
        Numpad3 => KEY_KP3,
        Numpad4 => KEY_KP4,
        Numpad5 => KEY_KP5,
        Numpad6 => KEY_KP6,
        Numpad7 => KEY_KP7,
        Numpad8 => KEY_KP8,
        Numpad9 => KEY_KP9,
        NumpadClear => KEY_RESERVED,
        NumpadEquals => KEY_KPEQUAL,
        NumpadDivide => KEY_KPSLASH,
        NumpadMultiply => KEY_KPASTERISK,
        NumpadMinus => KEY_KPMINUS,
        NumpadPlus => KEY_KPPLUS,
        NumpadEnter => KEY_KPENTER,
        NumpadDecimal => KEY_KPDOT,
        F1 => KEY_F1,
        F2 => KEY_F2,
        F3 => KEY_F3,
        F4 => KEY_F4,
        F5 => KEY_F5,
        F6 => KEY_F6,
        F7 => KEY_F7,
        F8 => KEY_F8,
        F9 => KEY_F9,
        F10 => KEY_F10,
        F11 => KEY_F11,
        F12 => KEY_F12,
        FastForward => KEY_FASTFORWARD,
        Rewind => KEY_REWIND,
        PlayPause => KEY_PLAYPAUSE,
        VolumeUp => KEY_VOLUMEUP,
        VolumeDown => KEY_VOLUMEDOWN,
        Mute => KEY_MUTE,
    }
}

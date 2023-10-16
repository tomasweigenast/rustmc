use std::any::Any;

use super::identifier::Identifier;

// Represents a Chat message defined at https://wiki.vg/Chat
#[derive(Debug)]
pub struct Chat {}

#[derive(Debug)]
pub struct ChatComponent {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
    pub font: Identifier,
    pub color: String,
    pub insertion: String,
    pub clickEvent: Box<ClickEvent>,
    pub hoverEvent: Box<HoverEvent>,
    pub extra: Vec<ChatComponent>,
}

#[derive(Debug)]
pub struct ClickEvent {
    pub action: ClickEventAction,
    pub value: dyn Any,
}

#[derive(Debug)]
pub enum ClickEventAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Debug)]
pub struct HoverEvent {
    pub action: HoverEventAction,
    pub contents: dyn Any,
}

#[derive(Debug)]
pub enum HoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

use objc2_app_kit::NSEvent;

pub(crate) fn shift_digit(event: &NSEvent) -> Option<u8> {
    let modifiers = crate::events::read_modifiers(event);
    if !modifiers.shift || modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
        return None;
    }
    const TOP_ROW: [u16; 10] = [29, 18, 19, 20, 21, 23, 22, 26, 28, 25];
    let key = event.keyCode();
    TOP_ROW.iter().position(|&code| code == key).map(|index| index as u8)
        .or_else(|| match key {
            82..=89 => Some((key - 82) as u8),
            91..=92 => Some((key - 83) as u8),
            _ => None,
        })
}

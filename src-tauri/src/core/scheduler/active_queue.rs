use crate::core::input::Key;

#[derive(Debug, Default)]
pub struct ActiveQueue {
    order: Vec<Key>,
    cursor: usize,
}

impl ActiveQueue {
    pub fn press(&mut self, key: Key) {
        self.order.retain(|existing| *existing != key);
        self.order.insert(0, key);
        self.cursor = 0;
    }

    pub fn release(&mut self, key: Key) {
        self.order.retain(|existing| *existing != key);
        if self.cursor >= self.order.len() {
            self.cursor = 0;
        }
    }

    pub fn next_key(&mut self) -> Option<Key> {
        if self.order.is_empty() {
            return None;
        }

        if self.cursor >= self.order.len() {
            self.cursor = 0;
        }

        let key = self.order[self.cursor];
        self.cursor = (self.cursor + 1) % self.order.len();
        Some(key)
    }

    pub fn order(&self) -> &[Key] {
        &self.order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_pressed_key_is_sent_first() {
        let mut queue = ActiveQueue::default();
        queue.press(Key::J);
        queue.press(Key::L);
        queue.press(Key::H);

        assert_eq!(queue.order(), &[Key::H, Key::L, Key::J]);
        assert_eq!(queue.next_key(), Some(Key::H));
        assert_eq!(queue.next_key(), Some(Key::L));
        assert_eq!(queue.next_key(), Some(Key::J));
        assert_eq!(queue.next_key(), Some(Key::H));
    }

    #[test]
    fn released_key_is_removed() {
        let mut queue = ActiveQueue::default();
        queue.press(Key::J);
        queue.press(Key::L);
        queue.press(Key::H);
        queue.release(Key::L);

        assert_eq!(queue.order(), &[Key::H, Key::J]);
        assert_eq!(queue.next_key(), Some(Key::H));
        assert_eq!(queue.next_key(), Some(Key::J));
    }
}

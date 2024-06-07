use crate::{print, println};
use alloc::{borrow::ToOwned, string::ToString};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;
use futures_util::{stream::Stream, StreamExt};
use pc_keyboard::{
    layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet, ScancodeSet1,
};

static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new() should only be called once.");
        ScancodeStream { _private: () }
    }
}

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: SCANCODE queue FULL. Dropping keyboard input!");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: SCANCODE queue uninitialized!");
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("ScanCode Queue not initialized.");
        // if let Some(scancode) = queue.pop() {
        //     Poll::Ready(Some(scancode))
        // }
        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.wake();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!(
                            "{} is your character {} as string",
                            character,
                            character.to_string().as_str()
                        )
                    }
                    DecodedKey::RawKey(key) => {
                        print!("{:?}", key)
                    }
                }
            }
        }
    }
}

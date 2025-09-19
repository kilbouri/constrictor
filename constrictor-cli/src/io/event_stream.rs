use crossterm::event::{Event, read};
use std::{io, sync::mpsc, thread};

/// Provides non-blocking access to a stream of [`Event`]s by creating a
/// background thread that passes [`Event`]s through an [`mpsc::channel`].
pub struct EventStream {
    recv: mpsc::Receiver<Result<Event, io::Error>>,
}

impl EventStream {
    // FIXME: this should really be singleton pattern
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            loop {
                if send.send(read()).is_err() {
                    // other side of channel has hung up! Oh noes!
                    return;
                }
            }
        });

        Self { recv }
    }
}

impl Iterator for EventStream {
    type Item = Result<Event, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv.try_iter().next()
    }
}

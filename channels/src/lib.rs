use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}};


struct Sender<T>
{
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T>
{
    fn send(&self, data: T)
    {
        let mut queue = self.shared.inner.lock().unwrap();
        queue.queue.push_back(data);
        drop(queue);
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T>
{
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        Sender {
            shared: Arc::clone(&self.shared),
        } 
    }
}

impl<T> Drop for Sender<T>
{
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
    }
}

struct Receiver<T>
{
    shared: Arc<Shared<T>>, 
}

impl<T> Receiver<T>
{
    fn recv(&self) -> Option<T>
    {
        let mut queue = self.shared.inner.lock().unwrap();
        loop {
            match queue.queue.pop_front() {
                Some(t) => return Some(t),
                None if queue.senders == 0 => return None,
                None => queue = self.shared.available.wait(queue).unwrap(),
            }
        }
    }
}

struct Inner<T>
{
    queue: VecDeque<T>,
    senders: u32,
}

struct Shared<T>
{
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

fn channel<T: Send>() -> (Sender<T>, Receiver<T>)
{
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };

    let shared = Arc::new ( 
        Shared {
            inner: Mutex::new(inner),
            available: Condvar::new(),
        }
    );

    (
        Sender {
            shared: shared.clone(),    
        },
        Receiver {
            shared: shared.clone(),    
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_works() {
        let (tx, rx) = channel();
        tx.send(14);
        assert_eq!(rx.recv(), Some(14));
    }

    #[test]
    fn multiple_senders() {
        let (tx, rx) = channel();
        let tx2 = tx.clone();
        tx2.send(14);
        tx.send(15);
        assert_eq!(rx.recv(), Some(14));
        assert_eq!(rx.recv(), Some(15));
    }

    #[test]
    fn drop_sender() {
        let (tx, rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn drop_receiver() {
        let (tx, rx) = channel();
        drop(rx);
        tx.send(14);
    }
}

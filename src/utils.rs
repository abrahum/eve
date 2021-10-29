use bytes::{Buf, Bytes};
use tracing::trace;

/// show cutted bytes if too long
pub fn log_trace_bytes(head: &str, data: &Bytes) {
    if data.remaining() > 20 {
        trace!(
            "{} bytes:{:x}...",
            head,
            Bytes::copy_from_slice(&data[0..20])
        );
    } else {
        trace!("{} bytes:{:x}", head, data)
    }
}

pub trait MapExt<T> {
    fn then_do<F>(self, f: F)
    where
        F: FnOnce(T);
}

impl<T> MapExt<T> for Option<T> {
    fn then_do<F>(self, f: F)
    where
        F: FnOnce(T),
    {
        match self {
            Some(t) => f(t),
            None => {}
        }
    }
}

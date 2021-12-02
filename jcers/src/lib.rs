#![doc = include_str!("../README.md")]

mod de;
mod err;
mod ser;
#[cfg(test)]
mod test;
mod util;
mod value;

use std::fmt::LowerHex;

use bytes::Buf;
pub use de::{Jce, JceGet};
pub use err::{JceError, JceResult};
// #[cfg(feature = "derive")]
// #[cfg_attr(docrs, doc(cfg(feature = "derive")))]
pub use jcers_proc::{JceGet, JcePut};
pub use ser::{JceMut, JcePut};
pub use util::{JceHead, JceType};
pub use value::*;

/// Read data from Buf
///
/// ## Example
/// ```rust
/// use bytes::Bytes;
/// let mut buf = Bytes::from(vec![0x00u8, 0x01u8]);
/// let v: JceValue = from_buf(&mut buf).unwrap();
/// assert_eq!(v, JceValue::Byte(0x01u8));
/// ```
pub fn from_buf<B, T>(buf: &mut B) -> JceResult<T>
where
    B: Buf + LowerHex,
    T: JceGet,
{
    let mut jce_mut = Jce::new(buf);
    Ok(T::jce_get(&mut jce_mut)?)
}

#[test]
fn test_from_buf() {
    use bytes::Bytes;
    let mut buf = Bytes::from(vec![0x00u8, 0x01u8]);
    let v: JceValue = from_buf(&mut buf).unwrap();
    assert_eq!(v, JceValue::Byte(0x01u8));
}

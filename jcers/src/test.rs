use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestStruct {
    pub a: u8,                  // 0
    pub b: i32,                 // 1
    pub c: f32,                 // 2
    pub d: String,              // 3
    pub e: HashMap<u8, String>, // 6
}

impl JceGet for TestStruct {
    fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut de::Jce<B>) -> JceResult<Self> {
        if jce.head.ty != JceType::Struct {
            return Err(JceError::ReadTypeError(JceType::Struct, jce.head.ty));
        }
        let mut sub_jce = jce.sub_jce();
        let r = TestStruct {
            a: sub_jce.get_by_tag::<u8>(0)?,
            b: sub_jce.get_by_tag::<i32>(1)?,
            c: sub_jce.get_by_tag::<f32>(2)?,
            d: sub_jce.get_by_tag::<String>(3)?,
            e: sub_jce.get_by_tag::<HashMap<u8, String>>(6)?,
        };
        jce.end_struct()?;
        Ok(r)
    }

    fn empty() -> JceResult<Self> {
        Ok(Self::default())
    }
}

impl JcePut for TestStruct {
    fn jce_put(self, jce_mut: &mut ser::JceMut, tag: u8) {
        jce_mut.put_head(10, tag);
        self.a.jce_put(jce_mut, 0);
        self.b.jce_put(jce_mut, 1);
        self.c.jce_put(jce_mut, 2);
        self.d.jce_put(jce_mut, 3);
        0u8.jce_put(jce_mut, 4);
        0u8.jce_put(jce_mut, 5);
        self.e.jce_put(jce_mut, 6);
        jce_mut.put_head(11, tag);
    }
}

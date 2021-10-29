use bytes::{Buf, Bytes};
static STRUCT_START_TAG: [u8; 1] = [0x0a];
static STRUCT_END_TAG: [u8; 1] = [0x0b];

pub async fn http_post(url: &str, data: Bytes) -> Result<Bytes, reqwest::Error> {
    let cli = reqwest::Client::new();
    let res = cli
        .post(url)
        .header("User-Agent", "QQ/8.2.0.1296 CFNetwork/1126")
        .header("Net-Type", "Wifi")
        .body(data)
        .send()
        .await?;
    let bytes = res.bytes().await?;
    Ok(bytes)
}

pub fn pack_jce_struct_data(data: Bytes) -> Bytes {
    let abyte = Bytes::from(&STRUCT_START_TAG[..]);
    let bbyte = Bytes::from(&STRUCT_END_TAG[..]);
    let mut c = abyte.chain(data).chain(bbyte);
    c.copy_to_bytes(c.remaining())
}

pub fn unpack_jce_struct_data(mut data: Bytes) -> Bytes {
    if data.get_u8() == STRUCT_START_TAG[0] {
        data.copy_to_bytes(data.remaining() - 1)
    } else {
        panic!("unpack error")
    }
}

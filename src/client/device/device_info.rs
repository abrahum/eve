use std::borrow::Cow;

use bytes::{BufMut, Bytes, BytesMut};
use quick_protobuf::MessageWrite;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::ClientProtocol;

static DEVICE_ICFO_FILE_NAME: &'static str = "device.json";

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub display:        String,
    pub product:        String,
    pub device:         String,
    pub board:          String,
    pub brand:          String,
    pub model:          String,
    pub bootloader:     String,
    pub finger_print:   String,
    pub boot_id:        String,
    pub proc_version:   String,
    pub base_band:      String,
    pub sim_info:       String,
    pub os_type:        String,
    pub mac_address:    String,
    pub ip_address:     std::net::Ipv4Addr,
    pub wifi_bssid:     String,
    pub wifi_ssid:      String,
    pub imei:           String,
    pub android_id:     String,
    pub apn:            String,
    pub vendor_name:    String,
    pub vendor_os_name: String,
    pub protocol:       ClientProtocol,
    pub version:        Version,

    /// [u8; 16]
    #[serde(skip)]
    pub imsi_md5:  Bytes,
    /// [u8; 16]
    #[serde(skip)]
    pub guid:      Bytes,
    /// [u8; 16]
    #[serde(skip)]
    pub tgtgt_key: Bytes,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub incremental: String,
    pub release:     String,
    pub codename:    String,
    pub sdk:         u32,
}

impl Default for DeviceInfo {
    /// default data for DeviceInfo(unrand)
    fn default() -> Self {
        DeviceInfo {
            display:        "MIRAI.123456.001".to_owned(),
            product:        "mirai".to_owned(),
            device:         "mirai".to_owned(),
            board:          "mirai".to_owned(),
            brand:          "mamoe".to_owned(),
            model:          "mirai".to_owned(),
            bootloader:     "unkown".to_owned(),
            finger_print:   "mamoe/mirai/mirai:10/MIRAI.200122.001/1234567:user/release-keys"
                .to_owned(),
            boot_id:        "cb886ae2-00b6-4d68-a230-787f111d12c7".to_owned(),
            proc_version:   "Linux version 3.0.31-cb886ae2 (android-build@xxx.xxx.xxx.xxx.com)"
                .to_owned(),
            base_band:      String::new(),
            sim_info:       "T-Mobile".to_owned(),
            os_type:        "android".to_owned(),
            mac_address:    "00:50:56:C0:00:08".to_owned(),
            ip_address:     std::net::Ipv4Addr::new(10, 0, 1, 3),
            wifi_bssid:     "00:50:56:C0:00:08".to_owned(),
            wifi_ssid:      "<unknown ssid>".to_owned(),
            imei:           "468356291846738".to_owned(),
            android_id:     "MIRAI.123456.001".to_owned(),
            apn:            "wifi".to_owned(),
            vendor_name:    "MIUI".to_owned(),
            vendor_os_name: "mirai".to_owned(),
            protocol:       ClientProtocol::IPad,
            version:        Version {
                incremental: "5891938".to_owned(),
                release:     "10".to_owned(),
                codename:    "REL".to_owned(),
                sdk:         29,
            },

            imsi_md5:  Bytes::default(),
            guid:      Bytes::default(),
            tgtgt_key: Bytes::default(),
        }
    }
}

impl DeviceInfo {
    /// generate a new random DeviceInfo
    pub fn new() -> Self {
        let mut device_info = Self::default();
        device_info.display = format!("MIRAI.{}.001", super::gen_rand_number(6));
        device_info.finger_print = format!(
            "mamoe/mirai/mirai:10/MIRAI.200122.001/{}:user/release-keys",
            super::gen_rand_string(7)
        );
        device_info.boot_id = String::from_utf8(gen_uuid().to_vec()).unwrap();
        device_info.proc_version = format!(
            "Linux version 3.0.31-{} (android-build@xxx.xxx.xxx.xxx.com)",
            super::gen_rand_string(8)
        );
        device_info.imsi_md5 = Bytes::copy_from_slice(
            &md5::compute({
                let mut rand_array = [0u8; 16];
                rand::thread_rng().fill_bytes(&mut rand_array);
                rand_array
            })
            .0,
        );
        device_info.imei = new_imei();
        device_info.android_id = hex::encode({
            let mut rand_array = [0u8; 8];
            rand::thread_rng().fill_bytes(&mut rand_array);
            rand_array
        });
        //Todo
        device_info.gen();
        device_info
    }

    /// generate and update guid && tgtgt_key
    pub fn gen(&mut self) {
        // guid
        self.guid = Bytes::copy_from_slice(
            &md5::compute(format!("{}{}", self.android_id, self.mac_address).into_bytes()).0,
        );

        // tgtgt_key
        let mut rand_array = [0u8, 16];
        rand::thread_rng().fill_bytes(&mut rand_array);
        let mut bytes = BytesMut::with_capacity(32);
        bytes.extend(rand_array);
        bytes.extend(self.guid.clone());
        let bytes = bytes.freeze().to_vec();
        self.tgtgt_key = Bytes::copy_from_slice(&md5::compute(bytes).0);
    }

    /// load DeviceInfo from json &str
    pub fn load(data: &str) -> Self {
        let mut device_info: Self = serde_json::from_str(data).expect("device data broken");
        device_info.gen();
        device_info
    }

    /// dump DeviceInfo to String
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// load from file, if file don't exists will generate a new DeviceInfo and save
    pub fn load_or_new() -> Self {
        use std::fs;
        use std::path::PathBuf;
        use std::str::FromStr;

        let file = PathBuf::from_str(DEVICE_ICFO_FILE_NAME).unwrap();
        info!("Loading DeviceInfo from file {}", DEVICE_ICFO_FILE_NAME);
        if file.exists() {
            let data =
                fs::read_to_string(&file).expect(&format!("{} is broken", DEVICE_ICFO_FILE_NAME));
            info!("Success load DeviceInfo form file");
            Self::load(&data)
        } else {
            info!(
                "{} not exists, generating new {}",
                DEVICE_ICFO_FILE_NAME, DEVICE_ICFO_FILE_NAME
            );
            let info = Self::new();
            let data = info.to_json();
            fs::write(&file, data).unwrap();
            info
        }
    }

    pub fn gen_protobuf(&self) -> Bytes {
        use protobuf::data::DeviceInfo as PbDeviceInfo;
        use quick_protobuf::Writer;

        let pb = PbDeviceInfo {
            bootloader:   Cow::from(self.bootloader.clone()),
            procVersion:  Cow::from(self.proc_version.clone()),
            codename:     Cow::from(self.version.codename.clone()),
            incremental:  Cow::from(self.version.incremental.clone()),
            fingerprint:  Cow::from(self.finger_print.clone()),
            bootId:       Cow::from(self.boot_id.clone()),
            androidId:    Cow::from(self.android_id.clone()),
            baseBand:     Cow::from(self.base_band.clone()),
            innerVersion: Cow::from(self.version.incremental.clone()),
        };
        let mut vec_mut = Vec::with_capacity(pb.get_size());
        let mut writer = Writer::new(&mut vec_mut);
        pb.write_message(&mut writer)
            .expect("failed to write protobuf message");
        Bytes::from(vec_mut)
    }
}

/// generate a new imei code
fn new_imei() -> String {
    let mut s = String::with_capacity(15);
    s.push_str("86"); // China area
    let mut sum = 8 + 1 + 2; // Luhn algorithm
    let mut rng = rand::thread_rng(); // rand data
    for i in 0..12 {
        let v = rng.gen_range(0..=9);
        s.push_str(&v.to_string());
        sum = if i % 2 == 0 {
            // Luhn algorithm
            sum + v
        } else {
            sum + (v * 2 / 10) + (v * 2 % 10)
        };
    }
    s.push_str(&(sum % 10).to_string()); // Luhn algorithm
    s
}

/// generate a new uuid
pub fn gen_uuid() -> Bytes {
    fn put_fn(b: &mut BytesMut, data: String) {
        b.put(data.as_bytes());
        b.put(&b"-"[..]);
    }

    let mut rand_array = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut rand_array);
    let mut uuid = BytesMut::with_capacity(36);
    put_fn(&mut uuid, hex::encode(&rand_array[0..4]));
    put_fn(&mut uuid, hex::encode(&rand_array[4..6]));
    put_fn(&mut uuid, hex::encode(&rand_array[6..8]));
    put_fn(&mut uuid, hex::encode(&rand_array[8..10]));
    uuid.put(hex::encode(&rand_array[10..16]).as_bytes());
    uuid.freeze()
}

#[test]
fn gen_uuid_test() {
    let uuid = gen_uuid();
    println!("{:?}-len:{}", uuid, uuid.len());
}

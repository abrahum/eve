#[test]
fn sso_address_resp_decode() {
    use super::*;
    use bytes::Bytes;
    static SSO_ADDRESS_RESP : &'static str = "6e477b1c09e193f1d6084a1e8a052c259f6cf4e608e614d5db262ba96fceb6e8a5d19d72860d0db5dfac554cb7de435e4400f9cd0e8b14f98d4020962c496393e6947f85adcaf00656782b9512713177b41d8489ddf8952766c9850639e329905911aa989d618be1e14b133d228f187efd0ae3a8bdec00c6028078862e965940ad8acc937c5522abf967de737632f19d4f27b5bdf34a2003b11d8547cd4f82c7f1fcb8ea8219ada296ca5ed38bc38b4bbb475f51974dffb85daf1a12b3be2d853ff7877eba722148612abe85535492dd955ae9ff2b6cd9bb570711acc0869cdab62f0aa7fb1caa9862abd1e3aca39a96a9b45116cc92a065c2736420cab691e540534307dcc2d872da82c35b03f7a94b0a9bd6fbf73caa002702c10c9af38616ed9e6c54912de021ace4d969ca264d7f9d94ea4913a1a2184e77b9a1bc850c38d7de55b82e21c2f0e45e0e12ab602c54641b20409d1013245f79ec151e1ee773b9cca9f6d172084d1a125b9ff0e3d5efac8f0ad9e4cdec94f9366e6346274b6c9994ae62f35060c961948b5aa23eb2402c008603fa2e416d7803c9e466a9658d5e3470abda44c9b00c985ee28ae01e2f837666b8b8c5fb5fdfd263fa4173c948cf282efcd779fc15e21dbc29d4fa826d92d75bd7ebf19036c621d2fb96e6940fcab78ebea48da089893374b67d8341a10d86c33a182a9354268a7a7e168bd5c4a33e8b6110cd7e838c9bf53b8869ddbe747daf94183627e8dab6f71c67f71c621eddce055eb20803c962433baa1c7b0d1caf8a8b8b10ed3d429adaa38afaf1effab224ea16454e6e99b35dbcc55bfa972b8511ca1dee7af4485e45a4c0b5a6940168678dc0471420e4d9c401a514f12423972d6deb28192e7e9f74987282a67c4e1ea7e987ebbac989a72c85faac5d2648a284dfdb9374dfd17887c4b31b3e676436bbd47c5a4258406552187f2ca3586c58b1187c979c9303561fd4308646f30908d8abb19bae79d493100c034e6b34b2cd810ddb700c1a2d5418f556a8c46ed8912b1ea507d289c350e0ac86e2b80cee31263c10f431353d049890e7bbb732fed29353e75c4172193f45c7260157ee6f5cf84f11c3183f9889e30e239cf2fae16f1d65f14b519f407f848bd014c7b95d61ef6393b137b6989979641af5325f53411090c54164148ce3f732e0cdaac01de3b8585594d1d2e8f76723d995497d2c314f377efbc3974363d031e5de9fa4799a42b4acc28f7b2834203a1f3fa00510604801c7777ae1f62e89e08b6a0248d46a2c055e93458498077b175a2f08313ee373d42b861a727935be574838cdc15e654c6a6d01143d23d072bf7ea93e1094d4c5330a1ee3ea421f78127b60ea65dc9628a4e2eb440f21daaa514d30dde9ba5eea69c19a8ae8c65b60bad64859692dcf9c947b99333c31b21418e2984b178ee61011c0a3cec9d97abc4511ee96599e49fa8dc7b27d38d1cac74071c0b840db24d6ad4f2923622f7482ca0de4dc35698b69bbb3692d85f698f629e1476ac54a35502098ebf607ec1ad0d053745c8685e5fa69f91694163ab89c3391b7e0856239486a3206465e521364b4de7d5e5e2124e8c4a2bf9ca140f2af2c65b5e5c0a29f2cbcaaf0cea569250c1ce9c9356f1ad0ace6c9f729319cd61b701d5b8ccdd4509bfed9c2dbdcb7f6e47c97570b6914b43fd14798337dd997ce6d3d520d3db2a1fb0d50730f89a96df64d5f383c7889c405bb28e261f56a4f22c8dd8d5c95ac5d8653bdaa67e826d0e4214559d70e5f82ce0846970399b11e875c08571165b16029c5c575bb21316c19269ffac6db84efb2aa35b1c877bd824e8d00d18aaf4a7dfec14d8bfa3a92029936aea0dd4eb9c3c8b3c633d2b59339ea5865997c5e52facb546a222fa27e9ec00063a620ab44550241f3a115fc6251ad8b244a3588d400ee18ca0769d2ae64e09b8a6c7e439a856093087930e442d036e6bcfeee0c7b6fa908ab1c6906552cf0cd6341ac14bc04ef45c4bd558d45fa51b0ca3465a9c681fbe24fec1d231a73080b067caa5ac0641e6da3e3d105bf6491752e308c026e94e616582150d3a8a52a7bac25d3880c8324c18a92bcdc73c1dfa7f3591754f8944ac6c32d86163edff93de1d3e435367714c390bcb6ee741032e1981450112a1a1303969008c5170c0774dd7a61952778ff2bd6abab52834b7e5fa1d4c2ae47026e4072bea7d4fa359c25b36658c0372b11ad1c5723fb9308ce31ec4a4e0c38244fdfe4918079";
    // static DE_DATA: &'static str = "0000063710032c3c4c560a436f6e666967487474706611487474705365727665724c6973745265737d000106050800010611487474705365727665724c6973745265731d000105ea0a100129000b0a160e3130392e3234342e3139382e3134211f9030014c5c600870018602737a96066f74686572730b0a160f3138302e3130322e3131312e313035211f9030014c5c6008700186027368960374656c0b0a160c3131332e39362e31322e3835211f9030014c5c600870018602737a960374656c0b0a160b31342e32322e332e3132322101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139332e323530205030014c5c600870018602746a960374656c0b0a160e3131342e3232312e3134382e34392136b030014c5c6008700186027368960374656c0b0a160c3131332e39362e31332e39352101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139322e323131211f9030014c5c600870018602746a960374656c0b0a160c3130312e39312e34322e3938205030014c5c6008700186027368960374656c0b0a16116d7366776966692e33672e71712e636f6d211f9030014c5c60087c86066f746865727396066f74686572730b0a160d34322e38312e3137322e323037205030014c5c600870018602746a960374656c0b39000b0a160e3130392e3234342e3139382e3134211f9030014c5c600870018602737a96066f74686572730b0a160f3138302e3130322e3131312e313035211f9030014c5c6008700186027368960374656c0b0a160c3131332e39362e31322e3835211f9030014c5c600870018602737a960374656c0b0a160b31342e32322e332e3132322101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139332e323530205030014c5c600870018602746a960374656c0b0a160e3131342e3232312e3134382e34392136b030014c5c6008700186027368960374656c0b0a160c3131332e39362e31332e39352101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139322e323131211f9030014c5c600870018602746a960374656c0b0a160c3130312e39312e34322e3938205030014c5c6008700186027368960374656c0b0a16116d7366776966692e33672e71712e636f6d211f9030014c5c60087c86066f746865727396066f74686572730b0a160d34322e38312e3137322e323037205030014c5c600870018602746a960374656c0b426155ae2b5138406c7c80029005acbcc900080a160e3130392e3234342e3132392e3135205030014c500360087c8602737a96066f74686572730b0a160e3131342e3232312e3134342e3232205030014c500360087c86027368960374656c0b0a160c3131332e39362e31332e3434205030014c500360087c8602737a960374656c0b0a160e3131392e3134372e3139302e3337205030014c500360087c8602737a960374656c0b0a160d34322e38312e3139332e323432205030014c500360087c8602746a960374656c0b0a160d3138302e3130322e35392e3530205030014c500360087c86027368960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0bd900080a160e3130392e3234342e3132392e3135205030014c500360087c8602737a96066f74686572730b0a160e3131342e3232312e3134342e3232205030014c500360087c86027368960374656c0b0a160c3131332e39362e31332e3434205030014c500360087c8602737a960374656c0b0a160e3131392e3134372e3139302e3337205030014c500360087c8602737a960374656c0b0a160d34322e38312e3139332e323432205030014c500360087c8602746a960374656c0b0a160d3138302e3130322e35392e3530205030014c500360087c86027368960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0bed000cf90f0cf9100cf9110cf01202f113ff38f61428323032312d30392d33302031363a33313a33392064656c6976657279696e67206120706f6c696379fc150b8c980ca80c";

    let key = hex::decode("F0441F5FF42DA58FDCF7949ABA62D411").unwrap();
    let tea = crypto::tea::Tea::new(Bytes::from(key));

    let data = hex::decode(SSO_ADDRESS_RESP).unwrap();
    let mut de_rsp = tea.decrypt(Bytes::from(data)).unwrap();

    let _ = de_rsp.split_to(4);
    let mut request_packet: RequestPacket = Jce::read_from_bytes(&mut de_rsp);

    let mut request_data_version3: RequestDataVersion3 =
        Jce::read_from_bytes(&mut request_packet.s_buffer);

    let sso_server_infos: HttpServerListRes = Jce::read_from_bytes(
        request_data_version3
            .map
            .get_mut("HttpServerListRes")
            .unwrap(),
    );
    for s in sso_server_infos.sso_server_infos {
        println!("Get Addrs server:{} port:{}", s.server, s.port);
    }
}

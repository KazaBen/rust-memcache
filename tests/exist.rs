extern crate memcache;

#[test]
fn test_raw() {
    let client = memcache::connect("localhost", 2333).unwrap();
    client.flush(0);

    assert!(!client.exist("foo").unwrap());

    client.set_raw("foo", &[0x1u8, 0x2u8, 0x3u8], 0, 42).unwrap();
    assert!(client.exist("foo").unwrap());
}

use mtproto::{EncryptedMessage, EncryptionHeader, Payload};

fn main() {
    let message = EncryptedMessage {
        header: EncryptionHeader {
            auth_key_id: 1045,
            msg_key: 0xF0DA5E,
        },
        encrypted_data: Payload {
            data: b"484684".to_vec(),
        },
    };

    let bytes: Vec<u8> = bincode::config()
        .little_endian()
        .serialize(&message)
        .unwrap();

    println!("{:?}", message);

    print!("0x");
    for (i, b) in bytes.iter().enumerate() {
        if i % 2 == 0 && i > 0 {
            print!("'");
        }
        print!("{:02x}", b);
    }

    println!("\n{}B", bytes.len());
}

use std::io::Write;

use i_am_mt::{
    transport::tcp_client::{TcpClient, TransporterVersion},
    utils::{MyResult, TlWriteBytes},
};

fn main() -> MyResult<()> {
    let request = PqReq {
        auth_key_id: 0, // Always 0
        message_id: rand::random(),
        message_length: 20,
        constructor: 0x60469778,
        nonce: rand::random(),
    };
    println!("request: {:#?}", request);

    let mut stream: TcpClient =
        TcpClient::connect("149.154.167.40:443".parse()?, TransporterVersion::Abridged)?;
    stream.send_package(request.encode().as_ref())?;
    println!("request data:");
    println!();
    show(request.encode().as_ref());

    let response = stream.recv_package()?;
    println!("response data:");
    show(response.as_ref());

    Ok(())
}

fn show(data: &[u8]) {
    for (index, i) in data.iter().enumerate() {
        if index % 16 == 0 {
            print!("{:04x} | ", index);
        }
        print!("{:02x} ", i);
        if index % 16 == 15 {
            println!()
        } else if index % 4 == 3 {
            print!("  ")
        } else if index % 2 == 1 {
            print!(" ")
        }
    }
    println!()
}

#[derive(Debug)]
pub struct PqReq {
    auth_key_id: u64,
    message_id: u64,
    message_length: u32,
    constructor: u32,
    nonce: [u8; 16],
}

impl PqReq {
    pub fn encode(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.write_u64(self.auth_key_id).unwrap();
        result.write_u64(self.message_id).unwrap();
        result.write_u32(self.message_length).unwrap();
        result.write_u32(self.constructor).unwrap();
        result.write_all(self.nonce.as_ref()).unwrap();

        result
    }
}

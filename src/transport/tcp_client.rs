use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::utils::{MyResult, TlReadBytes, TlWriteBytes};

pub struct TcpClient {
    stream: TcpStream,
    version: TransporterVersion,
}

pub enum TransporterVersion {
    // TODO Support full version
    Intermediate,
    Abridged,
}

impl TcpClient {
    pub fn connect(remote_address: SocketAddr, version: TransporterVersion) -> MyResult<Self> {
        let mut stream = TcpStream::connect(remote_address)?;

        match version {
            TransporterVersion::Intermediate => {
                stream.write_u32(0xee_ee_ee_ee)?;
            }
            TransporterVersion::Abridged => {
                stream.write_u8(0xef)?;
            }
        }

        Ok(TcpClient { stream, version })
    }

    pub fn send_package(&mut self, payload: &[u8]) -> MyResult<()> {
        match self.version {
            TransporterVersion::Intermediate => self.send_package_intermediate(payload),
            TransporterVersion::Abridged => self.send_package_abridged(payload),
        }
    }

    pub fn recv_package(&mut self) -> MyResult<Vec<u8>> {
        match self.version {
            TransporterVersion::Intermediate => self.recv_package_intermediate(),
            TransporterVersion::Abridged => self.recv_package_abridged(),
        }
    }

    fn recv_package_abridged(&mut self) -> MyResult<Vec<u8>> {
        let first_byte = self.stream.read_u8()? as u32;

        let length = if first_byte < 127 {
            first_byte as u32
        } else {
            self.stream.read_u24()?
        };
        let mut buffer = vec![0u8; (length * 4) as usize];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn recv_package_intermediate(&mut self) -> MyResult<Vec<u8>> {
        let length = self.stream.read_u32()?;
        let mut buffer = vec![0u8; length as usize];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn send_package_abridged(&mut self, payload: &[u8]) -> MyResult<()> {
        let size = (payload.len() / 4) as u32;
        if size < 127 {
            self.stream.write_u8(size as u8)?;
        } else {
            self.stream.write_u32(size << 8 | 127)?;
        }
        self.stream.write_all(payload)?;
        Ok(())
    }

    fn send_package_intermediate(&mut self, payload: &[u8]) -> MyResult<()> {
        self.stream.write_u32(payload.len() as u32)?;
        self.stream.write_all(payload)?;
        Ok(())
    }
}

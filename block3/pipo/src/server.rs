use crate::error::Result;
use std::{
    env,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, ToSocketAddrs},
};

/// pipo server
pub struct PipoServer {
    listener: TcpListener,
}

impl PipoServer {
    /// 实例化一个通过 tcp 协议监听 addr 的 PipoServer
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        println!(
            "{}\n{} server start at {}\n",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_NAME"),
            listener.local_addr()?,
        );
        Ok(Self { listener })
    }

    /// 启动 pipo 服务
    pub fn run(&self) -> Result<()> {
        loop {
            loop {
                let (stream, addr) = self.listener.accept()?;
                println!(
                    "{} a new TCP connection is established with {}",
                    env!("CARGO_PKG_NAME"),
                    addr
                );

                let mut reader = BufReader::new(stream.try_clone()?);
                let mut writer = BufWriter::new(stream.try_clone()?);

                loop {
                    let mut request = String::new();
                    reader.read_line(&mut request)?;
                    let response = request;
                    writer.write_all(response.as_bytes())?;
                    writer.flush()?;
                }
            }
        }
    }
}

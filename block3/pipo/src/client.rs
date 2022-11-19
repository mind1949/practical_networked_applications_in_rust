use crate::error::Result;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdin, Stdout, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// pipo client
pub struct PipoClient {
    local: Local,
    remote: Remote,
}

impl PipoClient {
    /// 实例化一个通过 tcp 连接 addrs 的 PipoClient
    pub fn connect<A: ToSocketAddrs>(addrs: A) -> Result<Self> {
        let local = Local::new();
        let remote = Remote::connect(addrs)?;
        Ok(Self { local, remote })
    }

    /// 启动 pipo client
    pub fn run(&mut self) -> Result<()> {
        // 从标准输入读取数据
        loop {
            self.local.write(self.prompt().as_bytes())?;

            let mut request = String::new();
            self.local.reader.read_line(&mut request)?;
            // 写入远程服务
            self.remote.write(request.as_bytes())?;
            // 从远程服务读取响应
            let mut response = String::new();
            self.remote.reader.read_line(&mut response)?;
            // 写入本地
            self.local.write(self.prompt().as_bytes())?;
            self.local.writer.write_all(response.as_bytes())?;
            self.local.writer.flush()?;
        }
    }

    /// 提示符
    fn prompt(&self) -> String {
        format!("{}@{}>", env!("CARGO_PKG_NAME"), self.remote.addr)
    }
}

struct Local {
    reader: BufReader<Stdin>,
    writer: BufWriter<Stdout>,
}

impl Local {
    /// 创建一个Local实例
    fn new() -> Self {
        let reader = BufReader::new(stdin());
        let writer = BufWriter::new(stdout());
        Self { reader, writer }
    }
    /// 写数据
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size = self.writer.write(buf)?;
        self.writer.flush()?;
        Ok(size)
    }
}

struct Remote {
    addr: String,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Remote {
    fn connect<A: ToSocketAddrs>(addrs: A) -> Result<Self> {
        let stream = TcpStream::connect(addrs)?;

        let addr = stream.peer_addr()?.to_string();
        let reader = stream.try_clone()?;
        let writer = stream;
        let reader = BufReader::new(reader);
        let writer = BufWriter::new(writer);
        Ok(Self {
            addr,
            reader,
            writer,
        })
    }

    /// 写数据
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size = self.writer.write(buf)?;
        self.writer.flush()?;
        Ok(size)
    }
}

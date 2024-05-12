use std::net::{TcpListener, SocketAddrV4, Ipv4Addr, Shutdown};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let cmd = std::env::args().skip(1).collect::<Vec<_>>();
    if cmd.is_empty() {
        return Ok(());
    }

    let port = 7821;
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let listener = TcpListener::bind(addr)?;
    loop {
        let (mut client, _) = listener.accept()?;

        let mut buf = [0; 3];
        let read = client.read(&mut buf)?;
        let msg = &buf[0..read];
        let Ok(msg) = std::str::from_utf8(msg) else {
            continue;
        };
        if msg.trim() != "run" {
            continue;
        }
        let child = std::process::Command::new(&cmd[0])
            .args(cmd.iter().skip(1))
            .spawn();
        let mut child = match child {
            Ok(child) => child,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        child.wait()?;
        client.shutdown(Shutdown::Both)?;
    }
}


use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    // 读取buffer
    let mut buf = [0; 512];

    for _ in 0..1000 {
        // 将数据读入buffer
        let bytes_read = stream.read(&mut buf)?;
        // 如果没有任何数据，则返回
        if bytes_read == 0 {
            return Ok(());
        }

        // 返回
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 创建tcp监听器
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // 处理请求的线程池
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // 轮训处理请求
    for stream in listener.incoming() {
        // 接受请求
        let stream = stream.expect("failed!");
        // 使用handle_client函数处理请求
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        // 将处理请求的线程放进线程池
        thread_vec.push(handle);
    }

    // 所有处理请求的线程join
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::thread::{self, JoinHandle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("/file.txt")?;
    writeln!(
        file,
        "e785a7d529d589f13e610548b54ac636e30ff4c4e4d834b903b460"
    )?;

    for i in 0..1000 {
        println!("Handle: {}", i);
        let handlers = (1..4)
            .map(|_| {
                thread::spawn(|| -> Result<_, std::io::Error> {
                    let file = File::open("/file.txt").unwrap();
                    let mmap = unsafe { memmap::Mmap::map(&file).unwrap() };
                    let mut cursor = std::io::Cursor::new(mmap.as_ref());
                    let mut buffer: [u8; 6] = [0; 6];
                    cursor.read_exact(&mut buffer)?;
                    Ok(buffer)
                })
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();

        for handler in handlers {
            match handler.join().unwrap() {
                Ok(data) => assert_eq!("e785a7", std::str::from_utf8(&data)?),
                Err(e) => panic!("Error: {:?}", e),
            }
        }
    }
    println!("ok!");
    Ok(())
}

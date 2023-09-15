use std::fs::File;
use std::thread::{self, JoinHandle};

use arrow2::io::ipc::{
    read,
    write::{self, WriteOptions},
};
use arrow2::{
    array::{Int32Array, Utf8Array},
    chunk::Chunk,
    datatypes::{DataType, Field, Schema},
};

// use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Int32Array::from(&(0..10000).map(|i| Some(i)).collect::<Vec<_>>());
    let b: Utf8Array<i32> = Utf8Array::from(
        &(0..10000)
            .map(|i| Some(format!("string{}", i)))
            .collect::<Vec<_>>(),
    );

    let schema = Schema::from(vec![
        Field::new("c1", DataType::Int32, true),
        Field::new("c2", DataType::Int32, true),
    ]);

    let chunk = Chunk::try_new(vec![a.boxed(), b.boxed()])?;

    let options = WriteOptions { compression: None };

    let file = File::create("/tmp/file.ipc")?;
    let mut writer = write::FileWriter::new(file, schema, None, options);

    writer.start()?;
    writer.write(&chunk, None)?;
    writer.finish()?;

    for i in 0..100 {
        println!("Handle sample {}", i);
        let handlers = (1..3)
            .map(|_| {
                thread::spawn(
                    || -> Result<_, Box<dyn std::error::Error + Send + 'static>> {
                        let file = File::open("/tmp/file.ipc").unwrap();
                        let mmap = unsafe { memmap::Mmap::map(&file).unwrap() };
                        let metadata =
                            read::read_file_metadata(&mut std::io::Cursor::new(mmap.as_ref()))
                                .unwrap();

                        Ok(metadata)
                    },
                )
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();

        for handler in handlers {
            // Wait for the thread to finish. Returns a result.
            match handler.join().unwrap() {
                Ok(metadata) => println!("Metadata: {:?}", metadata),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
    println!("ok!");
    Ok(())
}

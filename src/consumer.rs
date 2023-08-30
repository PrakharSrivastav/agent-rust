use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

use failure::Error;
use serde::Serialize;
use tokio::sync::mpsc::Receiver;

pub async fn file_sink<T: Debug + Serialize>(mut channel: Receiver<T>) -> Result<(), Error> {
    let mut file = File::create("foo.bin")?;
    loop {
        tokio::select! {
            // msg = channel.recv() => match msg {
            //     Some(item) => {
            //         //println!("{:?} ", item);
            //         file.write_all(&bincode::serialize(&item)?)? ;
            //     },
            //     None => ()
            // }
            msg = channel.recv() => if let Some(item) = msg {
                 //println!("{:?} ", item);
                 file.write_all(&bincode::serialize(&item)?)? ;
           }

        }
    }
}

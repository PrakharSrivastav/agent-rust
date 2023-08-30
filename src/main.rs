// #![allow(dead_code)]

use crate::model::Ctrl;
use failure::Fallible;

mod consumer;
mod helper;
mod model;
mod producer;
mod worker;

#[tokio::main]
async fn main() -> Fallible<()> {
    let w = worker::Worker::spawn()?;

    helper::delay(1000).await;

    w.send_ctrl_message(Ctrl::Health).await?;

    helper::delay(1000).await;

    w.send_ctrl_message(Ctrl::Quit).await?;

    helper::delay(1000).await;
    Ok(())
}

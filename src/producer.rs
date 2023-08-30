use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::helper::delay;
use crate::model::{Ctrl, CtrlResponse, HealthResponse, Message, QuitResponse};

pub async fn message_generator(
    mut ctrl_channel: Receiver<(Ctrl, oneshot::Sender<CtrlResponse>)>,
    message_channel: Sender<Message>,
) {
    loop {
        tokio::select! {
            msg = message_channel.send(Message::Hello) => match msg {
                Ok(()) => delay(100).await,
                Err(_) => {
                    eprint!("error.sending.message");
                    break;
                }
            },

            ctrl = ctrl_channel.recv() => match ctrl {
                None => break,
                Some((Ctrl::Quit,rtx)) => {
                    println!("received.quit.message");
                    rtx.send(CtrlResponse::Quit(QuitResponse::OK)).unwrap() ;
                    break;
                }   ,
                Some((Ctrl::Health,rtx)) => {
                    rtx.send(CtrlResponse::Health(HealthResponse::Healthy)).unwrap();
                }
            },
        }
    }
}

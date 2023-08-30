use failure::Fallible;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::oneshot;

use crate::model::{Ctrl, CtrlResponse};
use crate::{consumer, model, producer};

pub struct Worker {
    ctrl: Sender<(Ctrl, oneshot::Sender<CtrlResponse>)>,
}

impl Worker {
    pub fn spawn() -> Fallible<Worker> {
        let (msg_tx, msg_rx) = channel::<model::Message>(10);

        let (ctrl_tx, ctrl_rx) = channel::<(Ctrl, oneshot::Sender<CtrlResponse>)>(10);

        tokio::spawn(producer::message_generator(ctrl_rx, msg_tx));
        tokio::spawn(consumer::file_sink(msg_rx));

        Ok(Worker { ctrl: ctrl_tx })
    }

    pub async fn send_ctrl_message(&self, msg: Ctrl) -> Fallible<CtrlResponse> {
        let (health_sender, health_receiver) = oneshot::channel::<CtrlResponse>();
        self.ctrl.send((msg, health_sender)).await?;
        Ok(health_receiver.await?)
    }
}

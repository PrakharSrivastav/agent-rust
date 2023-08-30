use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum Message {
    Hello,
    World,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HealthResponse {
    Healthy,
    Unhealthy,
}

#[derive(Debug)]
pub enum Ctrl {
    Quit,
    Health,
}

#[derive(Debug)]
pub enum CtrlResponse {
    Quit(QuitResponse),
    Health(HealthResponse),
}

#[derive(Debug)]
pub enum QuitResponse {
    OK,
}

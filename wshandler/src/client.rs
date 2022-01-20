use crate::msg::WSMessage;
use crate::{user::User, wall::ProtectiveWall};
use axum::extract::ws::{Message, WebSocket};
use backtrace::Backtrace;
use crossbeam_channel::{bounded, tick, Receiver, Sender};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::thread;
use std::{fmt::Debug, time::Duration};
use uuid::Uuid;

const WRITE_WAIT: u64 = 10 * Duration::from_secs(1).as_secs();
const PONG_WAIT: u64 = 60 * Duration::from_secs(1).as_secs();
const PING_PERIOD: u64 = (PONG_WAIT * 9) / 10;
const MAX_MESSAGE_SIZE: u64 = 1024;
const MAX_SEND_CHAN: u64 = 1024;
const MAX_SEND_CHAN_CAPACITY: u64 = MAX_SEND_CHAN + 128;

static PING_FRAME: ClientCMD = ClientCMD {
    action: "ping".to_string(),
    args: Vec::new(),
    client_uuid: "".to_string(),
};

static PONG_FRAME: WSMessage = WSMessage::new("System", 0, "pong");

#[derive(Deserialize, Serialize, Debug)]
struct ClientCMD {
    pub action: String,
    args: Vec<String>,
    client_uuid: String,
}

#[derive(Debug)]
struct Client {
    uuid: String,
    conn: WebSocket, //ws.Conn
    user: Option<User>,
    disconnected: bool,
    sender_channel: Sender<WSMessage>,
    recv_channel: Receiver<WSMessage>,
    clientip: String,
    wall: ProtectiveWall,
}

impl Client {
    fn new() -> Self {
        let (s1, r1) = bounded(MAX_MESSAGE_SIZE as usize);
        Client {
            conn: todo!(),
            user: None,
            disconnected: false,
            sender_channel: s1.clone(),
            recv_channel: r1.clone(),
            clientip: "".to_string(),
            wall: ProtectiveWall::new(),
            uuid: Uuid::new_v4().to_string(),
        }
    }

    fn set_channel(&self) {
        let (s1, r1) = bounded(MAX_MESSAGE_SIZE as usize);
        self.sender_channel = s1.clone();
        self.recv_channel = r1.clone();
    }

    fn user_key(&self) -> String {
        if let Some(user) = self.user {
            user.user_key()
        } else {
            "".to_string()
        }
    }

    fn cache_key(&self) -> String {
        format!("{}_{}", self.user_key(), self.remote_addr())
    }

    fn set_user_info(&self, u: User) {
        self.user = Some(u)
    }

    fn is_authenticated(&self) -> bool {
        match self.user {
            Some(user) => true,
            None => false,
        }
    }

    fn close(&self) {
        if self.disconnected {
            return;
        }
        self.disconnected = true;
        let bt = Backtrace::new();
        debug!("client ip: {} , stack:{:?}", self.clientip, bt);
        drop(self.sender_channel);
    }

    fn ip(&self) -> String {
        return self.clientip;
    }

    fn remote_addr(&self) -> String {
        return self.clientip;
    }

    async fn read_pump(&mut self) {
        // todo need add auth ...
        loop {
            let cmd = self.do_socket_msg().await.unwrap();
            //debug!("Read %#v", cmd)
            // clientChan <- &cmd;
        }
    }

    async fn do_socket_msg(&mut self) -> Option<ClientCMD> {
        if let Some(msg) = self.conn.recv().await {
            if let Ok(msg) = msg {
                // todo parse msg to WSMessage, use json
                println!("Client says: {:?}", msg);
            } else {
                println!("client disconnected");
                return None;
            }
        }

        let random_uuid = Uuid::new_v4();
        let cmd = ClientCMD {
            action: todo!(),
            args: Vec::new(),
            client_uuid: random_uuid.to_string(),
        };
        // todo! decode msg to WSMessage
        debug!("accept message: {:?}, message id = {}", cmd, random_uuid.to_string());
        if cmd.action == PING_FRAME.action {
            debug!(
                "respond to ping message={:?}, message id = %{}",
                PING_FRAME, random_uuid
            );
            PONG_FRAME.data = format!("pong+{}", random_uuid);
            self.sender_channel.send(PONG_FRAME);
            debug!(
                "respond to ping message end, pong message:{}, message id = {}",
                PONG_FRAME.data, random_uuid
            );
        }
        cmd.client_uuid = self.uuid;
        Some(cmd)
    }

    async fn write_pump(&self) {
        let ticker = tick(Duration::from_secs(PING_PERIOD));
        // self.Close();

        loop {
            let ticker_recv = thread::spawn(|| async {
                ticker.recv().unwrap();
                if self.disconnected {
                    debug!("send message message failed");
                    return;
                };

                match self.conn.send(Message::Ping(b"PING".to_vec())).await {
                    Ok(msg) => (),
                    Err(err) => {
                        warn!("Client {} PingMessage error: {}", self.clientip, err);
                        return;
                    }
                };
            });
            let msg_recv = thread::spawn(|| async {
                let msg = self.recv_channel.recv();
                match msg {
                    Ok(msg) => {
                        debug!("send message start:{:?}", msg);
                        if self.disconnected {
                            warn!(
                                "Client {} disconnected error: {}, failed message:{:?}",
                                self.clientip, self.disconnected, msg
                            )
                        }
                        match self.conn.send(Message::Text(json!(&msg).to_string())).await {
                            Ok(()) => (),
                            Err(err) => {
                                warn!("Client {} WriteJSON error: {}", self.clientip, err);
                                return;
                            }
                        }
                        debug!("send message end:{:?}", msg);
                    }
                    Err(err) => {
                        warn!("Client {} WriteJSON error: {}", self.clientip, err);
                        return;
                    }
                }
            });
            ticker_recv.join();
            msg_recv.join();
        }
    }

    fn send_message(&self, msg: WSMessage) -> bool {
        if self.disconnected {
            return false;
        }
        // thread::spawn(move || self.sender_channel.send(msg).unwrap());
        true
        // select!{
        //  self.send <- msg:
        // 	return true
        // default:
        // 	warn!("Client %s SendMessage Full", self.clientip)
        // 	// ToDo 主动断线
        // 	return false
        // }
    }
}

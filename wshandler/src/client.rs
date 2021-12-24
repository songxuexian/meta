use std::time::{Duration, Instant};
use crossbeam_channel::{bounded, select, Sender, Receiver};

use crate::{user::User, wall::ProtectiveWall};
use log::{error, info, warn, debug};
use backtrace::Backtrace;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


const WRITE_WAIT: u64  = 10 * Duration::from_secs(1).as_secs();
const PONG_WAIT: u64 = 60 * Duration::from_secs(1).as_secs();
const PING_PERIOD: u64 = (PONG_WAIT * 9) / 10;
const MAX_MESSAGE_SIZE: u64 = 1024;
const MAX_SEND_CHAN: u64 = 1024;
const MAX_SEND_CHAN_CAPACITY: u64 = MAX_SEND_CHAN + 128;

static  PING_FRAME: ClientCMD = ClientCMD{action: "ping".to_string(), args: todo!(), client: todo!() };
static PONG_FRAME: WSMessage = WSMessage{group: "System".to_string(),data:  "pong".to_string(), uid: todo!(),};

struct ClientCMD  {
	action : String ,
	args   : Vec<String>, 
	client : Client,
}

struct WSMessage {
	group :String,
	uid   :i64,
	data  :String,
}


struct Client  {
	conn       :   String, //ws.Conn
	user       :  Option<User>,
	disconnected : bool,
	sender_channel : Sender<String>,
	recv_channel : Receiver<String>,
	clientip  :   String,
	wall       :  ProtectiveWall,
}

impl Client{

fn UserKey(&self)-> String {
	if let Some(user) = self.user {
		user.user_key()
	} else {
		"".to_string()
	}
}

fn CacheKey(&self) ->String {
    format!("{}_{}", self.UserKey(), self.remote_addr())
}

fn SetUserInfo(&self,u :User) {
	self.user = Some(u)
}

fn IsAuthenticated(&self)-> bool {
	match self.user{
		Some(user)=> true,
		None => false,
	}
}

fn Close(&self) {
	if self.disconnected {
		return
	}
	self.disconnected = true;
	let bt = Backtrace::new();
	debug!("client ip: {} , stack:{:?}",self.clientip, bt);
	drop(self.sender_channel);
}

fn Ip(&self) ->String {
	return self.clientip
}

fn remote_addr(&self) -> IpAddr {
	return self.conn.RemoteAddr()
}

fn readPump(&self) {
	defer func() {
		self.Close()
		if self.IsAuthenticated() {
			ClientOffline(self)
			offlineFunc(self.CacheKey())
		}
		loggers.Info.Printf("Client %s readPump closed %s", c.clientip, c.user)
	}()
	self.conn.SetReadLimit(MAX_MESSAGE_SIZE)
	self.conn.SetReadDeadline(time.Now().Add(PONG_WAIT))
	self.conn.SetPongHandler(func(string) error {
		self.conn.SetReadDeadline(time.Now().Add(pongWait))
		if self.IsAuthenticated() {
			aliveFunc(c.CacheKey())
		}
		return nil
	})
	for {
		let cmd  = ClientCMD{ action: todo!(), args: todo!(), client: todo!() };
		if err = self.conn.ReadJSON(&cmd); err != nil {
			//if ws.IsUnexpectedCloseError(err, ws.CloseGoingAway, ws.CloseAbnormalClosure) {
			warn!("Client {} ReadJSON error {}", c.clientip, err);
			//}
			return
		}
		let randomUUID = uuid.New();

		debug!("accept message: {}, message id = {}", cmd, randomUUID.String());
		debug!("c.sendChan={}, message id = {}", len(self.sendChan), randomUUID.String() );
		if cmd.Action == PING_FRAME.Action {
			debug!("respond to ping message={}, message id = %{}", PING_FRAME, randomUUID.String() );
			PONG_FRAME.Data = fmt.Sprintf("pong+%v", randomUUID);
			c.sendChan <- PONG_FRAME;
			debug!("respond to ping message end, pong message:%v, message id = {}",  PONG_FRAME.Data,randomUUID.String());
			continue
		}
		cmd.Client = self;
		//debug!("Read %#v", cmd)
		clientChan <- &cmd;
	}
}

fn writePump(&self,ctx:context.Context) {
	ticker = time.NewTicker(PING_PERIOD);
	defer func() {
		ticker.Stop();
		self.Close();

		info!("Client %s writePump closed %s", self.clientip, self.user);
	}();
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if c.disconnected {
				debug!("send message message failed");
				return
			}
			self.conn.SetWriteDeadline(time.Now().Add(writeWait))
			if err := self.conn.WriteMessage(ws.PingMessage, nil); err != nil {
				warn!("Client {} PingMessage error: {}", self.clientip, err);
				return
			}
		case msg, ok := <-c.sendChan:
			debug!("send message start:{}", msg);
			if self.disconnected {
				warn!("Client {} disconnected error: %v, failed message:{}", self.clientip, self.disconnected, msg);
				return
			}
			if self.conn.SetWriteDeadline(time.Now().Add(writeWait)); !ok {
				error!("Client {} read sendChan fail: {}", c.clientip);
				// The hub closed the channel.
				debug!("set write deadline failed message:%+v", msg);
				err := c.conn.WriteMessage(ws.CloseMessage, []byte{});
				if err != nil{
					error!("Client {} write error:{}", c.clientip, err);
				}
				return
			}
			debug!("WriteJSON msg {}", msg)
			if err := self.conn.WriteJSON(msg); err != nil {
				warn!("Client {} WriteJSON error: {}", c.clientip, err);
				return
			}
			debug!("send message end:{}", msg);
		}
	}

}

fn SendMessage(&self,msg: String) -> bool {
	if self.disconnected {
		return false
	}
	select {
	case self.send <- msg:
		return true
	default:
		warn!("Client %s SendMessage Full", c.clientip)
		// ToDo 主动断线
		return false
	}
}
}
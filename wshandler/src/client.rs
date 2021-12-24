use std::time::{Duration, Instant};

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
	conn       :  ws.Conn,
	user       :  User,
	disconnected : bool,
	sendChan   :  chan interface{},
	clientip  :   String,
	wall       :  ProtectiveWall,
}

impl Client{

fn UserKey(&self)-> String {
	if self.user != None {
		return self.user.UserKey()
	}
	return ""
}

fn CacheKey(&self) ->String {
    format!("{}_{}", self.UserKey(), self.RemoteAddr())
}

fn SetUserInfo(&self,u *User) {
	slef.user = u
}

fn IsAuthenticated(&self)-> bool {
	return self.user != nil
}

fn Close(&self) {
	if self.disconnected {
		return
	}
	self.disconnected = true
	loggers.Debug.Printf("client ip:%v , stack:%v",self.clientip, string(debug.Stack()))
	self.conn.Close()
}

fn Ip(&self) ->string {
	return self.clientip
}

fn RemoteAddr(&self) ->net.Addr {
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
		var cmd ClientCMD
		if err := self.conn.ReadJSON(&cmd); err != nil {
			//if ws.IsUnexpectedCloseError(err, ws.CloseGoingAway, ws.CloseAbnormalClosure) {
			loggers.Warn.Printf("Client %s ReadJSON error %s", c.clientip, err)
			//}
			return
		}
		randomUUID := uuid.New()

		loggers.Debug.Printf("accept message: %#v, message id = %v", cmd, randomUUID.String())
		loggers.Debug.Printf("c.sendChan=%d, message id = %v", len(self.sendChan), randomUUID.String() )
		if cmd.Action == PING_FRAME.Action {
			loggers.Debug.Printf("respond to ping message=%d, message id = %v", PING_FRAME, randomUUID.String() )
			PONG_FRAME.Data = fmt.Sprintf("pong+%v", randomUUID)
			c.sendChan <- PONG_FRAME
			loggers.Debug.Printf("respond to ping message end, pong message:%v, message id = %v",  PONG_FRAME.Data,randomUUID.String())
			continue
		}
		cmd.Client = self
		//loggers.Debug.Printf("Read %#v", cmd)
		clientChan <- &cmd
	}
}

fn writePump(&self,ctx:context.Context) {
	ticker = time.NewTicker(PING_PERIOD)
	defer func() {
		ticker.Stop()
		self.Close()

		loggers.Info.Printf("Client %s writePump closed %s", self.clientip, self.user)
	}()
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if c.disconnected {
				loggers.Debug.Println("send message message failed")
				return
			}
			self.conn.SetWriteDeadline(time.Now().Add(writeWait))
			if err := self.conn.WriteMessage(ws.PingMessage, nil); err != nil {
				loggers.Warn.Printf("Client %s PingMessage error: %v", self.clientip, err)
				return
			}
		case msg, ok := <-c.sendChan:
			loggers.Debug.Printf("send message start:%+v", msg)
			if self.disconnected {
				loggers.Warn.Printf("Client %s disconnected error: %v, failed message:%+v", self.clientip, self.disconnected, msg)
				return
			}
			if self.conn.SetWriteDeadline(time.Now().Add(writeWait)); !ok {
				loggers.Error.Printf("Client %s read sendChan fail: %v", c.clientip)
				// The hub closed the channel.
				loggers.Debug.Printf("set write deadline failed message:%+v", msg)
				err := c.conn.WriteMessage(ws.CloseMessage, []byte{})
				if err != nil{
					loggers.Error.Printf("Client %s write error: %v", c.clientip, err)
				}
				return
			}
			loggers.Debug.Printf("WriteJSON msg %#v", msg)
			if err := self.conn.WriteJSON(msg); err != nil {
				loggers.Warn.Printf("Client %s WriteJSON error: %v", c.clientip, err)
				return
			}
			loggers.Debug.Printf("send message end:%+v", msg)
		}
	}

}

fn SendMessage(&self,msg interface{}) ->bool {
	if self.disconnected {
		return false
	}
	select {
	case self.sendChan <- msg:
		return true
	default:
		loggers.Warn.Printf("Client %s SendMessage Full", c.clientip)
		// ToDo 主动断线
		return false
	}
}
}
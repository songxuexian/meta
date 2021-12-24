pub(crate) struct User{
	uid    : i64,
	dev     :String,
	user_key :String,
}

impl User {
    fn  String(&self) ->String {
        if self.uid < 1 {
            return "".to_string();
        }
         format!("{} {}", self.uid,self.dev)
    }
    
    pub fn  user_key(&self) ->String {
        if self.user_key == "" {
            self.user_key = self.GetUserKey(self.uid, self.dev)
        }
        return self.user_key
    }
    
    fn GetUserKey(&self,uid:i64, dev:String) ->String {
        if uid < 1 {
            return "".to_string();
        }
        
        format!("{} {}", self.uid, self.dev)
    }
}


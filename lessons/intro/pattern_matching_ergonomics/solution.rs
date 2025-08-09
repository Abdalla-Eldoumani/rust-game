pub enum Msg { Ping, Data(i32), Quit }

pub fn handle(msg: Msg) -> Result<i32, &'static str> {
    match msg {
        Msg::Ping => Ok(0),
        Msg::Data(x) if x > 0 => Ok(x),
        Msg::Data(_) => Err("neg"),
        Msg::Quit => Err("quit"),
    }
}



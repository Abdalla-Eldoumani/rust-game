pub enum Msg { Ping, Data(i32), Quit }

/// Handle message: return Ok(value) for Data>0, Err("neg") for Data<=0,
/// and Ok(0) for Ping; Quit returns Err("quit").
pub fn handle(msg: Msg) -> Result<i32, &'static str> {
    // TODO: match with guards
    Err("todo")
}



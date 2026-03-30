use std::io::Write;

use crate::{FunctionCall, Marshallable, Unmarshallable};

pub struct ClientStub<S: Write> {
    conn: S,
}

impl<S: Write> ClientStub<S> {
    pub fn send<Req: Marshallable + From<FunctionCall>, Res: Unmarshallable>(
        &mut self,
        func_call: FunctionCall,
    ) -> Res {
        let req: Req = func_call.into();
        self.conn.write_all(&req.marshall());
        todo!()
    }
}

pub struct ServerStub<Req: Unmarshallable, Res: Marshallable> {}

pub fn send(mut self, body: &[u8]) -> io::Result<()> {           //#A
   //…       #B
}
// #A self is a special case so the first passed in argument is body: &[u8]
// #B Hiding the code because it is not relevant at the moment.
//    You can find it here: https://github.com/hyperium/hyper/blob/master/src/server/response.rs

//…
fn hello(_: Request, res: Response<Fresh>) {     //#A
    res.send(
      b"<div>
         <img src=\"https://www.rust-lang.org/logos/rust-logo-blk.svg\">
      </div>"
    ).unwrap();
}
//…
// #A Same function as listing 2.24 but with an HTML response

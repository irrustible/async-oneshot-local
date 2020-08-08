use async_oneshot_local::*;
use futures_lite::*;

#[test]
fn success() {
    let (s,r) = oneshot::<bool>();
    assert_eq!((), s.send(true).unwrap());
    assert_eq!(Ok(true), future::block_on(r));
}

#[test]
fn close_sender() {
    let (s,r) = oneshot::<bool>();
    s.close();
    assert_eq!(Err(Closed()), future::block_on(r));
}

#[test]
fn close_receiver() {
    let (s,r) = oneshot::<bool>();
    r.close();
    assert_eq!(Err(Closed()), s.send(true));
}


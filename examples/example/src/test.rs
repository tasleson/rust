use std::{thread, time};
use failure::{self, Fail, Error};

fn run_self_test(address: String) -> Result<(), Error> {
    let client_address = address.clone();

    let child = thread::spawn(move || {
        if let Err(e) = ::run_server(address, 4) {
            panic!("error: {:#?}", e.cause());
        }
    });

    // give server time to start
    thread::sleep(time::Duration::from_secs(1));

    let ret = ::run_client(client_address);
    if let Err(e) = ret {
        eprintln!("error: {:#?}", e.cause());
        return Err(e.into());
    }

    if let Err(e) = child.join() {
        Err(failure::err_msg(format!("{:#?}", e)))
    } else {
        Ok(())
    }
}

#[test]
fn test_unix() {
    assert!(run_self_test("unix:/tmp/io.systemd.network".into()).is_ok());
}

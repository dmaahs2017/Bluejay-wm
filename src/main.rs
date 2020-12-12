use xcb;

struct BlueJayWm {
    conn: xcb::Connection,
}

impl BlueJayWm {
}

fn start_bluejay() -> BlueJayWm {
    let (conn, _) = xcb::Connection::connect(None).expect("Connection to be created");

    BlueJayWm {
        conn
    }
}

fn main() {
    start_bluejay();

    println!("Hello, world!");
}

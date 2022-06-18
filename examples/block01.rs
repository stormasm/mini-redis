use mini_redis::blocking_client;

fn main() {
    let mut client = blocking_client::connect("localhost:6379").unwrap();

    client.set("foo", "bar".into()).unwrap();

    // Getting the value immediately works
    let val = client.get("foo").unwrap().unwrap();
    assert_eq!(val, "bar");
}

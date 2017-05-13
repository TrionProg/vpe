#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

extern crate uuid;
extern crate redis;
extern crate cdrs;
extern crate postgres;

pub use uuid::Uuid;
use redis::Commands;
use cdrs::query::QueryBuilder;
use cdrs::types::IntoRustByName;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data{
    s:String,
    pub id:Uuid,
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    let bin_data:Option<Vec<u8>> = con.get("45545").unwrap();

    let data=match bin_data {
        Some( bin_data ) => {
            //we use bincode to get data from redis
            let d:Data=bincode::deserialize(&bin_data).unwrap();
            d
        },
        None => {
            Data{
                s:"nothing".to_string(),
                id:Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),//and we can parse uuid as string or generate
            }
        },
    };

    //we use postgresql
    let tls_mode = postgres::TlsMode::None;
    let postgresql_connection = postgres::Connection::connect("postgresql://postgresql_user@localhost/users",tls_mode).unwrap();

    let insert_result=postgresql_connection.execute(
        "INSERT INTO users (id) VALUES ($1)",
        &[&data.id]
    ).unwrap();


    //We use cassandra
    let cassandra_address="127.0.0.1:9042";
    let cassandra_transport = cdrs::transport::TransportTcp::new(cassandra_address).unwrap();
    let cassandra_client = cdrs::client::CDRS::new(cassandra_transport, cdrs::authenticators::NoneAuthenticator);
    let mut cassandra_session = cassandra_client.start(cdrs::compression::Compression::None).unwrap();

    let cassandra_querry=QueryBuilder::new(format!("SELECT content from files WHERE id = {};",&data.id)).finalize();
}

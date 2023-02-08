#[derive(Debug)]
pub(crate) struct NewWebConnection(pub tokio::net::TcpStream);

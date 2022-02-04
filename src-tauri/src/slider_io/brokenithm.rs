use std::{convert::Infallible, net::SocketAddr};

use log::info;
use tokio::time::sleep;

use hyper::{
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Body, Request, Response, Server,
};

// use crate::slider_io::worker::{AsyncJob, AsyncJobFut, AsyncJobRecvStop};

async fn handle_request(
  request: Request<Body>,
  remote_addr: SocketAddr,
) -> Result<Response<Body>, Infallible> {
  Ok(Response::new(Body::from(format!(
    "Hello there connection {}\n",
    remote_addr
  ))))
}

async fn brokenithm_server() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 1666));

  info!("Brokenithm opening on {:?}", addr);

  let make_svc = make_service_fn(|conn: &AddrStream| {
    let remote_addr = conn.remote_addr();
    async move {
      Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
        handle_request(request, remote_addr)
      }))
    }
  });

  let server = Server::bind(&addr).serve(make_svc);
  if let Err(e) = server.await {
    eprintln!("Server error: {}", e);
  }
}

// struct BrokenithmJob;

// impl AsyncJob {
//   fn job(self, mut recv_stop: AsyncJobRecvStop) -> AsyncJobFut {
//     return Box::pin()
//   }
// }

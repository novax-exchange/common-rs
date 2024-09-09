use novax_tokio::tokio as tokio;

use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;
use core::future::Future;

pub async fn http_svc<F>(app: Router, addr: String, f: F) -> Result<(), Box::<dyn Error> > 
    where F:  Future<Output = ()> + Send + 'static
{
    let addr = addr.parse::<std::net::SocketAddr>()?;
    let listener = TcpListener::bind(&addr).await?;
    let rslt = match axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(f)
        .await 
    {
        Ok(rslt) => Ok( rslt ),
        Err(e) => Err(e.into())
    };
    rslt
}

// re-export
pub use axum;

use std::{pin::Pin, time::Duration};

use async_stream::try_stream;
use iced::futures::Stream;
use protos::image::{
    image_server::{Image, ImageServer},
    Empty, ImageBytes,
};
use tokio::{fs, signal, time};
use tonic::{transport::Server, Request, Response, Status};

pub mod protos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse()?;
    let server = Server::builder()
        .add_service(ImageServer::new(ImageService))
        .serve(address);
    let ctrl_c = signal::ctrl_c();

    tokio::select! {
        result = server => println!("Server {result:?}"),
        result = ctrl_c => println!("Ctrl-C {result:?}"),
    }

    Ok(())
}

struct ImageService;

#[tonic::async_trait]
impl Image for ImageService {
    type ImageStreamStream = Pin<Box<dyn Stream<Item = Result<ImageBytes, Status>> + Send>>;
    async fn image_stream(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ImageStreamStream>, Status> {
        let mut images = fs::read_dir("assets/images")
            .await
            .map_err(|err| Status::not_found(err.to_string()))?;
        let image_stream = try_stream! {
                    while let Some(entry) = images.next_entry()
                        .await
                        .map_err(|err| Status::not_found(err.to_string()))?
                    {
                        let path = entry.path();
                        if path.metadata().unwrap().len() <= 3 * 1024 * 1024 {
                            yield fs::read(path)
                                .await
                                .map(|bytes| ImageBytes { image: bytes })
                                .map_err(|err| Status::not_found(err.to_string()))?;
                            time::sleep(Duration::from_secs(2)).await;
                        }
                    }
                };

        Ok(Response::new(Box::pin(image_stream)))
    }
}

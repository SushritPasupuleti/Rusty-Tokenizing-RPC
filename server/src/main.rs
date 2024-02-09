use token::{
    token_server::{Token, TokenServer}, //TokenServer is auto-generated from the proto file and
    //so is `token_server` module.
    TokenRequest,
    TokenResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod token {
    tonic::include_proto!("token");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("token_descriptor");
}

#[derive(Debug, Default)]
pub struct TokenService {}

#[tonic::async_trait]
impl Token for TokenService {
    async fn process_token(
        &self,
        request: Request<TokenRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = token::TokenResponse {
            token: "Hello, world!".into(),
            error: "No error".into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let token_service = TokenService::default();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(token::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection)
        .add_service(TokenServer::new(token_service))
        .serve(address)
        .await?;
    Ok(())
}

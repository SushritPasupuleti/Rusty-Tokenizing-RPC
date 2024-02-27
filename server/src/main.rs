use token::{
    token_server::{Token, TokenServer}, //TokenServer is auto-generated from the proto file and
    //so is `token_server` module.
    TokenRequest,
    TokenResponse,
};

use tonic::{metadata::MetadataValue, transport::Server, Request, Response, Status};

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
            tokens: tokenize(request.into_inner().token.as_str()),
            error: "No error".into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let token_service = TokenService::default();
    let authenticated_token_service = TokenService::default();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(token::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .accept_http1(true) // <- Add grpc-web support
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(reflection)
        // .add_service(TokenServer::new(token_service)) // <- unauthenticated | switch for testing
        // .add_service(tonic_web::enable(TokenServer::new(token_service))) // <- grpc-web support + unauthenticated | switch for testing
        .add_service(TokenServer::with_interceptor(
            authenticated_token_service,
            check_auth,
        ))
        .serve(address)
        .await?;
    Ok(())
}

fn tokenize(input: &str) -> Vec<String> {
    //split into words
    let words: Vec<&str> = input.split_whitespace().collect();

    //convert to lowercase
    let words: Vec<String> = words.iter().map(|word| word.to_lowercase()).collect();

    //remove punctuation
    let words: Vec<String> = words
        .iter()
        .map(|word| word.replace(|c: char| !c.is_alphanumeric(), ""))
        .collect();

    //remove stop words
    let stop_words = vec![
        "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in", "is",
        "it", "its", "of", "on", "that", "the", "to", "was", "were", "will", "with",
    ];

    let words: Vec<String> = words
        .iter()
        .filter(|word| !stop_words.contains(&word.as_str()))
        .map(|word| word.to_string())
        .collect();

    return words;
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let req_token = req.metadata().get("authorization").unwrap();

    let token_str = req_token.to_str().unwrap();

    if token_str.starts_with("Bearer ") {
        let token = token_str.trim_start_matches("Bearer ");
        //TODO: check token against a database or something
        if token == "mytoken" {
            return Ok(req);
        } else {
            return Err(Status::unauthenticated("Invalid token"));
        }
    } else {
        return Err(Status::unauthenticated("Invalid token"));
    }
}

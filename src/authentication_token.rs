use std::future::{ Ready, ready };
use actix_web::{
    FromRequest,
    HttpRequest,
    http::header::HeaderValue,
    dev::Payload,
    Error as ActixWebError,
    error::ErrorUnauthorized,
};
use serde::{ Serialize, Deserialize };
use jsonwebtoken::{
    Validation,
    DecodingKey,
    decode,
};


#[derive(Serialize, Deserialize,Debug)]
pub struct Claims {
    pub customerid: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
	let req = req.clone();

	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);

	// No Header was sent
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }

	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();
    // println!("{:?}", authentication_token);
	// Couldn't convert Header::Authorization to String
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has foreign chars!"))) }

	// TODO put secret in app_state
    // let secret: &str = "seaker-secret-key";
	// let secret: &str = "secret";
    // let secret: &str = &req.app_data::<web::Data<String>>().unwrap();
    let authentication_token = authentication_token.replace("Bearer ","");
    let decode_token = decode::<Claims>(&authentication_token,&DecodingKey::from_secret("token-secret".as_ref()),&Validation::default());
    // println!("inside auth {:?}",decode_token);
	// let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
	//     &authentication_token,
	//     &DecodingKey::from_secret("seaker-secret".as_ref()),
	//     &Validation::new(Algorithm::HS512),
	// );
	match decode_token {
	    Ok(token) => ready(Ok(AuthenticationToken { id: token.claims.customerid })),
	    Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent! "))),
	}
    }
}

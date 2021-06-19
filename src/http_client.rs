pub mod http_client_module {
    use actix_web::client::{Client, Connector};
    use openssl::ssl::{SslConnector, SslMethod};

    pub async fn get(url: String) -> String {
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let client = Client::builder()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();
        let result = client.get(url).send().await.unwrap().body().await.unwrap();

        result.iter().map(|&s| s as char).collect::<String>()
    }
}

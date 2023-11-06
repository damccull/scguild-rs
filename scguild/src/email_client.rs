use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;

use crate::services::newsletter::domain::SubscriberEmail;

#[derive(Debug)]
pub struct EmailClient {
    authorization_token: Secret<String>,
    base_url: String,
    http_client: Client,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            authorization_token,
            http_client,
            base_url,
            sender,
        }
    }

    /// Send an email.
    #[tracing::instrument(skip(self, html_content, text_content))]
    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
        message_stream: Option<&str>,
    ) -> Result<(), reqwest::Error> {
        //TODO: Replace this with Url::join() eventually
        let url = format!("{}/email", self.base_url);

        let mut request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body: html_content,
            text_body: text_content,
            message_stream: None,
        };

        if let Some(stream) = message_stream {
            request_body.message_stream = Some(stream);
        }

        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
    message_stream: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use fake::{
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
        Fake, Faker,
    };
    use secrecy::Secret;
    use wiremock::{
        matchers::{any, header, header_exists, method, path},
        Mock, MockServer, Request, ResponseTemplate,
    };

    use crate::{email_client::EmailClient, services::newsletter::domain::SubscriberEmail};

    struct SendEmailBodyMatcher;
    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            // Try to parse the body's json
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                // Ensure mandatory fields are populated
                let mandatory = body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some();

                let strm = body.get("MessageStream");
                println!("The message stream is {:?}", &strm);
                // Ensure the correct stream if a stream is specified
                let correct_stream = match strm {
                    Some(stream) if stream == "test-stream" => true,
                    Some(serde_json::Value::Null) => true,
                    Some(_) => false,
                    None => true,
                };
                mandatory && correct_stream
            } else {
                false
            }
        }
    }

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// Generate some random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    /// Get a test instance of EmailClient
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let _ = email_client
            .send_email(&email(), &subject(), &content(), &content(), None)
            .await;

        // Assertions will happen when the mock server goes out of scope
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request_to_the_expected_stream() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        // Test correct stream
        let _ = email_client
            .send_email(
                &email(),
                &subject(),
                &content(),
                &content(),
                Some("test-stream"),
            )
            .await;

        // Test incorrect stream
        let _ = email_client
            .send_email(
                &email(),
                &subject(),
                &content(),
                &content(),
                Some("wrong-stream"),
            )
            .await;

        // Assertions will happen when the mock server goes out of scope
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content(), None)
            .await;

        // Assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content(), None)
            .await;

        // Assert
        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        // Delay 3 minutes before responding
        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content(), None)
            .await;

        assert_err!(outcome);
    }
}

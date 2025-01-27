#[derive(Clone, Debug, Default)]
pub struct AuthInterceptor {
    auth: Option<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>,
}

impl AuthInterceptor {
    /// Enable HTTP basic authentication with a username and optional password.
    pub fn basic<U, P>(username: U, password: Option<P>) -> Self
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        use base64::prelude::BASE64_STANDARD;
        use base64::write::EncoderWriter;
        use std::io::Write;

        let mut buf = b"Basic ".to_vec();
        {
            let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
            let _ = write!(encoder, "{username}:");
            if let Some(password) = password {
                let _ = write!(encoder, "{password}");
            }
        }
        let mut header = tonic::metadata::MetadataValue::try_from(buf)
            .expect("base64 is always valid HeaderValue");
        header.set_sensitive(true);

        Self { auth: Some(header) }
    }

    /// Enable HTTP bearer authentication.
    pub fn bearer<T>(token: T) -> Self
    where
        T: std::fmt::Display,
    {
        let header_value = format!("Bearer {token}");
        let mut header = tonic::metadata::MetadataValue::try_from(header_value)
            .expect("token is always valid HeaderValue");
        header.set_sensitive(true);

        Self { auth: Some(header) }
    }
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        if let Some(auth) = self.auth.clone() {
            request
                .metadata_mut()
                .insert(http::header::AUTHORIZATION.as_str(), auth);
        }
        Ok(request)
    }
}

impl tonic::service::Interceptor for &mut AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        if let Some(auth) = self.auth.clone() {
            request
                .metadata_mut()
                .insert(http::header::AUTHORIZATION.as_str(), auth);
        }
        Ok(request)
    }
}

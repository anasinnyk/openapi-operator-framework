pub struct ApiRequest<T> {
    pub request: reqwest::RequestBuilder,
    marker: std::marker::PhantomData<T>,
}

impl<T> ApiRequest<T> {
    pub fn new(request: reqwest::RequestBuilder) -> Self {
        Self {
            request,
            marker: std::marker::PhantomData,
        }
    }
}

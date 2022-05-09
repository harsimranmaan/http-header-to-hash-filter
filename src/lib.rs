use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
mod hmac;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeaderToHashRoot) });
}}

struct HttpHeaderToHashRoot;

impl Context for HttpHeaderToHashRoot {}

impl RootContext for HttpHeaderToHashRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        if let Some(header_value_raw) = &self.get_http_request_header("h1") {
            let hmac_value = hmac::calculate_hmac("secret", header_value_raw);
            info!("#{} -> {}: {}", self.context_id, "h1", hmac_value);
            self.set_http_request_header("h2", Some(hmac_value.as_str()));
        }
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

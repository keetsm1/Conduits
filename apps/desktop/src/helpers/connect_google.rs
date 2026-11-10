// Blocking + a background thread: gpui's executor is not a tokio runtime, so
// async reqwest would panic with "no reactor running".
pub fn connect_google() -> Result<String, String> {
    let response = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:8000/connect/google")
        .send()
        .map_err(|error| format!("Could not reach the backend: {error}"))?;

    let status = response.status();
    let body = response.text().map_err(|error| error.to_string())?;

    if status.is_success() {
        Ok(body)
    } else if body.is_empty() {
        Err(format!("HTTP {status}"))
    } else {
        // Backend sends the failure reason as plain text; show it as-is.
        Err(body)
    }
}

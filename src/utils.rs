use yew::services::fetch::{Request};
use serde::ser;
use failure;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Result<String, failure::Error>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_string(&body)?;
    Ok(Request::from_parts(parts, Ok(body)))
}

pub fn get_empty_body() -> Result<String, failure::Error> {
    Ok("".to_owned())
}

pub fn seriablizable_body<T>(body: &T) -> Result<String, failure::Error> where T: ser::Serialize {
    let body = serde_json::to_string(body)?;
    Ok(body)
}
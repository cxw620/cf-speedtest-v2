//! cf-speedtest-v2 library, or wasm entry

use worker::{
    event, Context, EncodeBody, Env, Request, Response, ResponseBody, ResponseBuilder, Result,
};

/// Brotli Data: 50MB
static BR_50M_BYTE: &[u8] = include_bytes!("../assets/50mb.test");
/// Brotli Data: 100MB
static BR_100M_BYTE: &[u8] = include_bytes!("../assets/100mb.test");
/// Brotli Data: 200MB
static BR_200M_BYTE: &[u8] = include_bytes!("../assets/200mb.test");
/// Brotli Data: 300MB
static BR_300M_BYTE: &[u8] = include_bytes!("../assets/300mb.test");
/// Brotli Data: 500MB
static BR_500M_BYTE: &[u8] = include_bytes!("../assets/500mb.test");
/// Brotli Data: 1000MB
static BR_1G_BYTE: &[u8] = include_bytes!("../assets/1gb.test");
/// Server version
pub static VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

#[inline(always)]
/// Get the brotli bomb body
pub fn body(path: impl AsRef<str>) -> Option<&'static [u8]> {
    Some(match path.as_ref() {
        "/50mb.test" => BR_50M_BYTE,
        "/100mb.test" => BR_100M_BYTE,
        "/200mb.test" => BR_200M_BYTE,
        "/300mb.test" => BR_300M_BYTE,
        "/500mb.test" => BR_500M_BYTE,
        "/1gb.test" => BR_1G_BYTE,
        _ => return None,
    })
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Ok(ResponseBuilder::new()
        .with_encode_body(EncodeBody::Manual)
        .with_header("x-server", VERSION)?
        .with_header("content-encoding", "br")?
        .body(ResponseBody::Body(
            body(req.path()).unwrap_or(BR_50M_BYTE).to_vec(),
        )))
}

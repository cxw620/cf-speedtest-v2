//! CF-SPEED-BR

use worker::*;

/// Brotli Data: 100MB
static BR_100M_BYTE: &[u8] = include_bytes!("../assets/100mb.test");
/// Brotli Data: 200MB
static BR_200M_BYTE: &[u8] = include_bytes!("../assets/200mb.test");
/// Brotli Data: 1000MB
static BR_1G_BYTE: &[u8] = include_bytes!("../assets/1gb.test");
/// Server version
static VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    console_debug!("accept request: {:#?}", &req);

    let body = match req.path().as_str() {
        "/100mb.test" => BR_100M_BYTE,
        "/200mb.test" => BR_200M_BYTE,
        "/1gb.test" => BR_1G_BYTE,
        _ => {
            console_warn!("unknown request path");
            return Response::error(String::new(), 404);
        }
    };

    Ok(ResponseBuilder::new()
        .with_encode_body(EncodeBody::Manual)
        .with_header("x-server", VERSION)?
        .with_header("content-encoding", "br")?
        .body(ResponseBody::Body(body.to_vec())))
}

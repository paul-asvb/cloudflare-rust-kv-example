use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result as SerdeResult};

use worker::*;

mod utils;
#[derive(Serialize, Deserialize)]
struct TestStruct {
    test_bool: bool,
    test_string: String,
    test_int: u64,
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/kv/:name", get_handler)
        .post_async("/kv/:name", post_handler)
        .get_async("/template", get_template)
        .get("/version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
async fn get_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.kv("KV_FROM_RUST") {
        Ok(store) => {
            if let Some(name) = ctx.param("name") {
                let res = store.get(name).await;
                if res.is_ok() {
                    let value = res.unwrap();
                    if value.is_some() {
                        return Response::ok(value.unwrap().as_string());
                    } else {
                        return Response::error("no value", 404);
                    }
                } else {
                    return Response::error("storage error", 500);
                }
            } else {
                return Response::error("no name defined", 400);
            }
        }
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };
}

async fn post_handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv("KV_FROM_RUST") {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let name = match ctx.param("name") {
        Some(name) => name,
        _ => return Response::error("no name defined", 400),
    };

    let content: String = match req.text().await {
        Ok(b) => b,
        _ => return Response::error("body parse error", 400),
    };

    //let json = serde_json::to_string(&my_struct);

    let put = store.put(name, content);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            return Response::ok("success");
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}
async fn get_template(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let reg = Handlebars::new();

    match reg.render_template("Hello {{name}}", &json!({"name": "foo"})) {
        Ok(tmp) => Response::ok(tmp),
        _ => Response::error("render_template error", 400),
    }
}

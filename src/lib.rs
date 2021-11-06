use worker::*;

mod utils;

#[derive(Serialize, Deserialize, Debug)]
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
        .get_async("/kv", handler)
        .post_async("/kv", handler)
        .get("/version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
async fn handler(mut _req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.kv("KV_FROM_RUST") {
        Ok(store) => {
            return Response::ok(format!("{:?}", store.list()));
        }
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    //Response::ok("success")
}

async fn create_handler(mut _req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.kv("KV_FROM_RUST") {
        Ok(store) => {
            store.put(
                "test",
                "TestStruct {
                    test_bool: tre,
                    test_string: todo.to_string(),
                    test_int: 1234,
                },"
                .to_string(),
            );
            return Response::ok(format!("{:?}", store.list()));
        }
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    //Response::ok("success")
}

// let store = match ctx.kv("NS") {
//     Ok(store) => {
//         console_log!("{:?}", store.list());
//         store
//     }
//     _ => return Response::error("store not found", 204),
// };
// match store.get("key").await {
//     Ok(Some(kv)) => Response::ok(format!("{:?}", kv)),
//     _ => Response::error("key not found", 204),
// }

//let kv = ctx.kv("paul-bot-NS")?;
//kv.put("key", "value")?.execute().await?;

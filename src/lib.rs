use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result as SerdeResult};
use yew;

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
        .get_async("/app", get_app)
        .get("/version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}

async fn get_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv("KV_FROM_RUST") {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let name = match ctx.param("name") {
        Some(n) => n,
        None => return Response::error("no name defined", 400),
    };

    match store.get(name).await {
        Ok(Some(value)) => Response::ok(value.as_string()),
        _ => Response::error("store.get(name) err", 500),
    }
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


enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: yew::ComponentLink<Self>,
    value: i64,
}

impl yew::Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

let print_hello = |name: String| {
    println!( "Hello, {}!", name );
};

js! {
    var print_hello = @{print_hello};
    print_hello( "Bob" );
    print_hello.drop(); // Necessary to clean up the closure on Rust's side.
}

async fn get_app(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
}

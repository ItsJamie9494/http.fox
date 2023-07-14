use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct GlobalCtx<'a> {
    title: &'a str,
    base_url: &'a str,
}

const GLOBAL_CONTEXT: GlobalCtx<'static> = GlobalCtx {
    title: "http.fox",
    base_url: "http://127.0.0.1",
};

/// Create a new Context object with the Global context set
pub fn context() -> GlobalCtx<'static> {
    GLOBAL_CONTEXT
}

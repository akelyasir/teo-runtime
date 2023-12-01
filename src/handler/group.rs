use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::Handler;
use crate::handler::handler::{Method};
use crate::middleware::next::Next;
use crate::request;
use crate::utils::next_path;

#[derive(Serialize, Debug)]
pub struct Group {
    pub path: Vec<String>,
    pub handlers: BTreeMap<String, Handler>,
}

impl Group {

    pub fn define_handler<T, F>(&mut self, name: &str, call: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let wrapped_call = Box::leak(Box::new(call));
        let handler = Handler {
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            format: HandlerInputFormat::Json,
            path: next_path(&self.path, name),
            ignore_prefix: false,
            method: Method::Post,
            url: None,
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                wrapped_call.call(ctx).await
            })),
        };
        self.handlers.insert(name.to_owned(), handler);
    }
}

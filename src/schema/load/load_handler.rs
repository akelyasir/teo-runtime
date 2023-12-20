use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::handler::handler::Method;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;

pub fn load_handler(main_namespace: &mut Namespace, schema: &Schema, handler_declaration: &teo_parser::ast::handler::HandlerDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    if let Some(mut handler) = main_namespace.handler_at_path(&handler_declaration.str_path()).cloned() {
        handler.format = handler_declaration.input_format;
        handler.input_type = handler_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone());
        handler.output_type = handler_declaration.output_type().resolved().clone();
        for decorator in handler_declaration.decorators() {
            let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
            if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, handler_declaration, main_namespace)?;
                (decorator_implementation.call)(args, &mut handler)?;
            }
        }
        if (handler.method != Method::Post) || handler.url.is_some() {
            main_namespace.handler_map.add_record(
                &handler_declaration.namespace_str_path(),
                handler_declaration.parent_string_path().last().unwrap().as_str(),
                handler_declaration.name(),
                handler.method,
                handler.url.as_ref().map(|u| u.as_str()),
                handler.ignore_prefix,
            );
        }
        main_namespace.replace_handler_at_path(&handler_declaration.str_path(), handler);
    }
    Ok(())
}
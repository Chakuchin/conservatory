use utoipa_swagger_ui::SwaggerUi;
use utoipa::openapi::OpenApi;

pub fn init(api: OpenApi) -> SwaggerUi {
        SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", api)
}
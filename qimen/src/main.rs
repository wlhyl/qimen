use actix_web::{
    middleware::{self, Logger},
    web, App, HttpServer,
};
use actix_web_validator::JsonConfig;
use qimen::{
    args,
    routers::{health_routes, qimen_routes},
    state::AppState,
    Error,
};
use std::{env, net::SocketAddrV4};

#[cfg(feature = "cors")]
use actix_cors::Cors;

use clap::Parser;

#[cfg(feature = "swagger")]
use qimen::swagger::ApiDoc;

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let log4rs_config = env::var("LOG4RS_CONFIG")
        .expect("没设置 LOG4RS_CONFIG 环境变量，可在.env文件中设置或export LOG4RS_CONFIG=...");

    log4rs::init_file(&log4rs_config, Default::default())
        .map_err(|error| format!("配置日志系统失败，日志配置文件：{log4rs_config}, {error}"))
        .unwrap();

    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let shared_data = web::Data::new(AppState { ephe_path });

    let args = args::Args::parse();

    #[cfg(feature = "swagger")]
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        #[cfg(feature = "cors")]
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let json_config = JsonConfig::default()
            // .limit(4096)
            // .content_type(|mime| {  // <- accept text/plain content type
            //     mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
            // })
            .error_handler(|err, _req| {
                // <- create custom error response
                // error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
                Error::from(err).into()
            });

        let app = App::new()
            // .app_data(QsQueryConfig::default().error_handler(|err, _| {
            //     let json_error = match &err {
            //         Error::Validate(error) => ValidationErrorJsonPayload::from(error),
            //         _ => ValidationErrorJsonPayload { message: err.to_string(), fields: Vec::new() },
            //     };
            //     error::InternalError::from_response(err, HttpResponse::Conflict().json(json_error)).into()
            // }))
            .app_data(json_config)
            .app_data(shared_data.clone())
            .configure(health_routes)
            .service(web::scope("/api").configure(qimen_routes));

        #[cfg(feature = "swagger")]
        let app = app.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        );

        #[cfg(feature = "cors")]
        let app = app.wrap(cors);

        let app = app
            .wrap(Logger::default())
            .wrap(middleware::Compress::default());
        app
    })
    .workers(args.n)
    .bind(SocketAddrV4::new(args.ip, args.port))?
    .run()
    .await
}

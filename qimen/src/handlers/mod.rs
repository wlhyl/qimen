use actix_web::{post, web, HttpResponse, Responder};

use crate::{request::QiMenRequest, state::AppState, Error, QiMemShiPan};

pub mod healthz;

/// 阴盘奇门排盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    context_path="/api",
    request_body=QiMenRequest,
    responses(
        (status = 200, description = "返回阴盘奇门盘", body = QiMemShiPan),
    ),
)
)]
#[post("/qimen")]
pub async fn qimen(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<QiMenRequest>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    // let native_date = horo_date_time(
    //     r.year,
    //     r.month,
    //     r.day,
    //     r.hour,
    //     r.minute,
    //     r.second,
    //     8.0,
    //     false,
    // )?;

    let pan = QiMemShiPan::new(
        r.year,
        r.month,
        r.day,
        r.hour,
        r.minute,
        r.second,
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Ok().json(pan))
}

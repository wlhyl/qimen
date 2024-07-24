use utoipa::OpenApi;

use crate::{handlers::__path_qimen, request::QiMenRequest, Gan, QiMemShiPan, Shen, Star, Men};

use ganzhiwuxing::GanZhi;
use lunar_calendar::LunarCalendar;

use horo_date_time::HoroDateTime;

// swagger
#[derive(OpenApi)]
#[openapi(
    paths(qimen),
    components(schemas(
        HoroDateTime,
        GanZhi,
        LunarCalendar,
        Gan,Shen,Star,Men,
        QiMenRequest,
        QiMemShiPan,
        

        // DiZhi,GanZhi,TianGan,WuXing,
        // 农历
        // LunarCalendar,SolarTerm,
        // ZiWei,
        // DateRequest, ZiWeiRenReust,
        // star
        // Star,StarCategory,StarName,Power, Vstar,
        // StarCategory
    ))
)]
pub struct ApiDoc;

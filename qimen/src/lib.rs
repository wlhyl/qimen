pub mod args;
mod error;
mod gan;
mod handlers;
mod men;
mod request;
pub mod routers;
mod shen;
mod star;
pub mod state;

#[cfg(feature = "swagger")]
pub mod swagger;

pub use gan::Gan;
use gan::Gan::*;
use men::Men::{self, *};
use star::Star::{self, *};

use ganzhiwuxing::{DiZhi, TianGan};
use horo_date_time::{horo_date_time, HoroDateTime};
use lunar_calendar::{lunar_calendar, LunarCalendar};

pub use error::Error;
use serde::Serialize;
use shen::Shen::{self, *};
use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path, Body};

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Debug, Serialize)]
pub struct QiMemShiPan {
    /// 起局时间
    pub date: HoroDateTime,
    /// 农历日期
    pub lunar_calendar: LunarCalendar,
    /// 局数，阳数：正数，阴遁：负数
    pub ju_shu: i8,
    pub kong: [DiZhi; 2],
    /// 马星
    pub ma: DiZhi,
    /// 地盘
    // #[cfg_attr(feature = "swagger", schema(example=json!([["戊"], ["己"], ["庚"], ["辛"], ["壬"], ["癸", "丁"], ["丙"], ["乙"]])))]
    pub di_pan: Vec<Vec<Gan>>,
    /// 天盘
    // #[cfg_attr(feature = "swagger", schema(example=json!([["戊"], ["己"], ["庚"], ["辛"], ["壬"], ["癸", "丁"], ["丙"], ["乙"]])))]
    pub tian_pan: Vec<Vec<Gan>>,
    /// 神盘
    pub shen_pan: Vec<Shen>,
    /// 值符
    pub zhi_fu: Star,
    /// 星盘
    pub star_pan: Vec<Star>,
    /// 值使
    pub zhi_shi: Men,
    /// 门盘
    pub men_pan: Vec<Men>,
    /// 隐干盘
    pub yin_gan_pan: Vec<Vec<Gan>>,
    // /// 地盘入墓
    // pub di_pan_mu: Vec<String>,
    // /// 天盘入墓
    // pub tian_pan_mu: Vec<String>,
    // /// 隐干盘墓
    // pub yin_gan_pan_mu: Vec<String>,
}

impl QiMemShiPan {
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        ephe_path: &str,
    ) -> Result<Self, Error> {
        let lunar = lunar_calendar(year, month, day, hour, minute, second, ephe_path)
            .map_err(|error| Error::Function(format!("计算农历错误：{error}")))?;

        // 计算局数
        let year_num = lunar.lunar_year_gan_zhi.zhi().minus(&DiZhi::亥);
        let month_num = lunar.lunar_month.to_num();
        let day_num = lunar.lunar_day.to_num();
        let time_num = lunar.time_gan_zhi.zhi().minus(&DiZhi::亥);

        let year_num = if year_num == 0 { 12 } else { year_num };
        let time_num = if time_num == 0 { 12 } else { time_num };

        let ju_shu = (year_num + month_num + day_num + time_num) % 9;

        let ju_shu = if ju_shu == 0 { 9 } else { ju_shu as i8 };

        // 计算太阳的黄道经度，用于判断阳遁或阴遁
        let date = horo_date_time(year, month, day, hour, minute, second, 8.0, false)?;
        swe_set_ephe_path(ephe_path);
        let xx = swe_calc_ut(date.jd_utc, &Body::SeSun, &[])
            .map_err(|e| Error::Function(format!("函数sun_on_asc计算太阳位置错误:{e}")))?;
        swe_close();

        let sun_long = xx[0];

        // 冬至，太阳在27.0
        let ju_shu = if swe_degnorm(sun_long - 270.0) < 180.0 {
            ju_shu
        } else {
            -ju_shu
        };

        let d = lunar.time_gan_zhi.gan().minus(&TianGan::甲) as isize;
        let xiu_shou = lunar.time_gan_zhi.plus(-d);

        let kong = [xiu_shou.zhi().plus(-2), xiu_shou.zhi().plus(-1)];

        let dun_jia_gan = match xiu_shou.zhi() {
            DiZhi::子 => 戊(false),
            DiZhi::戌 => 己(false),
            DiZhi::申 => 庚(false),
            DiZhi::午 => 辛(false),
            DiZhi::辰 => 壬(false),
            _ => 癸(false),
        };

        let di_pan_jiu_gong: Vec<_> = (0..9)
            .map(|x| {
                let n = if ju_shu > 0 {
                    (x + ju_shu) % 9
                } else {
                    (x + ju_shu - 9) % 9
                };

                if n == 0 {
                    9
                } else {
                    n.abs() as u8
                }
            })
            .collect();

        let gan_seq = [
            戊(false),
            己(false),
            庚(false),
            辛(false),
            壬(false),
            癸(false),
            丁(false),
            丙(false),
            乙(false),
        ];

        let mut di_pan: Vec<Vec<Gan>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        di_pan_jiu_gong.iter().enumerate().for_each(|(index, &x)| {
            // match x {
            //     1 => 0,
            //     8 => 1,
            //     3 => 2,
            //     4 => 3,
            //     9 => 4,
            //     2|5 => 5,
            //     7 => 6,
            //     _ => 7, //9=>7
            // };

            match x {
                1 => di_pan[0].push(gan_seq[index]),
                8 => di_pan[1].push(gan_seq[index]),
                3 => di_pan[2].push(gan_seq[index]),
                4 => di_pan[3].push(gan_seq[index]),
                9 => di_pan[4].push(gan_seq[index]),
                2 | 5 => di_pan[5].push(gan_seq[index]),
                7 => di_pan[6].push(gan_seq[index]),
                _ => di_pan[7].push(gan_seq[index]), //9=>7
            };
        });

        let time_gan_index_on_di_pan = di_pan
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<String>>())
            .position(|x| {
                x.contains(&{
                    let gan = lunar.time_gan_zhi.gan();
                    let gan = if gan == TianGan::甲 {
                        dun_jia_gan.to_string()
                    } else {
                        gan.to_string()
                    };
                    gan
                })
            })
            .unwrap();

        // 时辰旬首在地盘的位置
        let du_jia_gan_index_on_di_pan = di_pan
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<String>>())
            .position(|x| x.contains(&dun_jia_gan.to_string()))
            .unwrap();

        // 天盘坎位对应的地盘index
        let tian_pan_kan_gong_index =
            (du_jia_gan_index_on_di_pan + 8 - time_gan_index_on_di_pan) % 8;

        let tian_pan = [
            &di_pan[tian_pan_kan_gong_index..],
            &di_pan[..tian_pan_kan_gong_index],
        ]
        .concat();

        let shen_pan = if ju_shu > 0 {
            [值符, 塍蛇, 太阴, 六合, 白虎, 玄武, 九地, 九天]
        } else {
            [值符, 九天, 九地, 玄武, 白虎, 六合, 太阴, 塍蛇]
        };

        // 神盘坎宫的index，相对于上面的默认位置
        let shen_pan_index = 8 - time_gan_index_on_di_pan;
        let shen_pan = [&shen_pan[shen_pan_index..], &shen_pan[..shen_pan_index]].concat();

        // 九星
        let stars = [天蓬, 天任, 天冲, 天辅, 天英, 天苪, 天柱, 天心];
        // let time_gan_index_on_tian_pan = time_gan_index_on_di_pan;
        // let star_index = (du_jia_gan_index_on_di_pan + 8 - time_gan_index_on_tian_pan) % 8;
        let star_index = tian_pan_kan_gong_index;
        let star_pan = [&stars[star_index..], &stars[..star_index]].concat();
        let zhi_fu = stars[du_jia_gan_index_on_di_pan];

        // 门
        let mens = [休门, 生门, 伤门, 杜门, 景门, 死门, 惊门, 开门];
        let men_nums = [1, 8, 3, 4, 9, 2, 7, 6];
        let zhi_shi = mens[du_jia_gan_index_on_di_pan];

        // let men_num = men_nums[du_jia_gan_index_on_di_pan];

        // 如果遁甲干可能在5宫，直接在di_pan_jiu_gong找出遁甲干的九宫数
        // 如果遁甲干在5宫，八门仍从五宫飞
        let men_num = di_pan_jiu_gong
            .iter()
            .enumerate()
            .find_map(|(index, &x)| {
                if gan_seq[index].to_string() == dun_jia_gan.to_string() {
                    Some(x)
                } else {
                    None
                }
            })
            .unwrap();

        let n = lunar.time_gan_zhi.gan().minus(&TianGan::甲);
        let men_num = if ju_shu > 0 {
            (men_num + n) % 9
        } else {
            (men_num + 9 - n) % 9
        };

        let men_num = if men_num == 0 { 9 } else { men_num };

        // let zhi_shi_men_index = if men_num ==5{5}else{ men_nums.iter().position(|&x| x == men_num).unwrap()};
        let zhi_shi_men_index = if let Some(x) = men_nums.iter().position(|&x| x == men_num) {
            x
        } else {
            5 // 如果men_num==5,即值使门落入中五宫，则寄于二宫，二宫坐标为5
        };

        let mem_index = (du_jia_gan_index_on_di_pan + 8 - zhi_shi_men_index) % 8;
        let men_pan = [&mens[mem_index..], &mens[..mem_index]].concat();

        // 引干盘
        let yin_gan_pan = if lunar.time_gan_zhi.gan() == TianGan::甲 {
            // dun_jia_gan
            let yin_pan_jiu_gong: Vec<u8> = (0..9)
                .map(|x| {
                    let n = if ju_shu > 0 {
                        (x + 5) % 9
                    } else {
                        (5 + 9 - x) % 9
                    };

                    if n == 0 {
                        9
                    } else {
                        n
                    }
                })
                .collect();

            // 确定，引起序列
            // 如果遁甲干在5宫，按九宫飞，引干仍会伏吟，需要将坤二宫的地盘干（也可以是天盘干，因为此时天盘干与地盘干相）
            // 放入中五宫，按九宫飞

            // 找出地盘中五宫的天干
            let gan_on_di_pan_5_gong = di_pan_jiu_gong
                .iter()
                .enumerate()
                .find_map(|(index, &x)| if x == 5 { Some(gan_seq[index]) } else { None });
            let gan_on_di_pan_5_gong = gan_on_di_pan_5_gong.unwrap();

            let n = if gan_on_di_pan_5_gong.to_string() == dun_jia_gan.to_string() {
                // 找出坤二宫的index
                di_pan_jiu_gong.iter().position(|&x| x == 2).unwrap()
            } else {
                gan_seq
                    .iter()
                    .position(|x| x.to_string() == dun_jia_gan.to_string())
                    .unwrap()
            };

            // 引起序列，起于5宫
            let yin_gan_seq = [&gan_seq[n..], &gan_seq[..n]].concat();

            // if yin_gan_seq[0]

            let mut yin_gan_pan: Vec<Vec<Gan>> = vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ];

            yin_pan_jiu_gong.iter().enumerate().for_each(|(index, &x)| {
                match x {
                    1 => yin_gan_pan[0].push(yin_gan_seq[index]),
                    8 => yin_gan_pan[1].push(yin_gan_seq[index]),
                    3 => yin_gan_pan[2].push(yin_gan_seq[index]),
                    4 => yin_gan_pan[3].push(yin_gan_seq[index]),
                    9 => yin_gan_pan[4].push(yin_gan_seq[index]),
                    2 | 5 => yin_gan_pan[5].push(yin_gan_seq[index]),
                    7 => yin_gan_pan[6].push(yin_gan_seq[index]),
                    _ => yin_gan_pan[7].push(yin_gan_seq[index]), //9=>7
                };
            });

            yin_gan_pan
        } else {
            let yin_gan_index = (time_gan_index_on_di_pan + 8 - zhi_shi_men_index) % 8;
            [&di_pan[yin_gan_index..], &di_pan[..yin_gan_index]].concat()
        };

        // 入墓
        // 地盘入墓
        let di_pan = di_pan
            .into_iter()
            .enumerate()
            .map(|(index, gans)| match index {
                1 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "丁" {
                            丁(true)
                        } else if g.to_string() == "己" {
                            己(true)
                        } else if g.to_string() == "庚" {
                            庚(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                3 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "辛" {
                            辛(true)
                        } else if g.to_string() == "壬" {
                            壬(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                5 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "癸" {
                            癸(true)
                        } else if g.to_string() == dun_jia_gan.to_string() {
                            match dun_jia_gan {
                                戊(_) => 戊(true),
                                己(_) => 己(true),
                                庚(_) => 庚(true),
                                辛(_) => 辛(true),
                                壬(_) => 壬(true),
                                癸(_) => 癸(true),
                                丁(_) => 丁(true),
                                丙(_) => 丙(true),
                                乙(_) => 乙(true),
                            }
                        } else {
                            g
                        }
                    })
                    .collect(),
                7 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "乙" {
                            乙(true)
                        } else if g.to_string() == "丙" {
                            丙(true)
                        } else if g.to_string() == "戊" {
                            戊(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                _ => gans,
            })
            .collect();

        // 天盘入墓
        let tian_pan = tian_pan
            .into_iter()
            .enumerate()
            .map(|(index, gans)| match index {
                1 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "丁" {
                            丁(true)
                        } else if g.to_string() == "己" {
                            己(true)
                        } else if g.to_string() == "庚" {
                            庚(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                3 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "辛" {
                            辛(true)
                        } else if g.to_string() == "壬" {
                            壬(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                5 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "癸" {
                            癸(true)
                        } else if g.to_string() == dun_jia_gan.to_string() {
                            match dun_jia_gan {
                                戊(_) => 戊(true),
                                己(_) => 己(true),
                                庚(_) => 庚(true),
                                辛(_) => 辛(true),
                                壬(_) => 壬(true),
                                癸(_) => 癸(true),
                                丁(_) => 丁(true),
                                丙(_) => 丙(true),
                                乙(_) => 乙(true),
                            }
                        } else {
                            g
                        }
                    })
                    .collect(),
                7 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "乙" {
                            乙(true)
                        } else if g.to_string() == "丙" {
                            丙(true)
                        } else if g.to_string() == "戊" {
                            戊(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                _ => gans,
            })
            .collect();

        // 引干盘入墓
        let yin_gan_pan = yin_gan_pan
            .into_iter()
            .enumerate()
            .map(|(index, gans)| match index {
                1 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "丁" {
                            丁(true)
                        } else if g.to_string() == "己" {
                            己(true)
                        } else if g.to_string() == "庚" {
                            庚(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                3 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "辛" {
                            辛(true)
                        } else if g.to_string() == "壬" {
                            壬(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                5 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "癸" {
                            癸(true)
                        } else if g.to_string() == dun_jia_gan.to_string() {
                            match dun_jia_gan {
                                戊(_) => 戊(true),
                                己(_) => 己(true),
                                庚(_) => 庚(true),
                                辛(_) => 辛(true),
                                壬(_) => 壬(true),
                                癸(_) => 癸(true),
                                丁(_) => 丁(true),
                                丙(_) => 丙(true),
                                乙(_) => 乙(true),
                            }
                        } else {
                            g
                        }
                    })
                    .collect(),
                7 => gans
                    .into_iter()
                    .map(|g| {
                        if g.to_string() == "乙" {
                            乙(true)
                        } else if g.to_string() == "丙" {
                            丙(true)
                        } else if g.to_string() == "戊" {
                            戊(true)
                        } else {
                            g
                        }
                    })
                    .collect(),
                _ => gans,
            })
            .collect();

        // 马星
        let ma = [DiZhi::寅, DiZhi::巳, DiZhi::申, DiZhi::亥]
            .iter()
            .find_map(|zhi| {
                if lunar.time_gan_zhi.zhi().minus(zhi) % 4 == 0 {
                    Some(zhi.plus(6))
                } else {
                    None
                }
            });

        let ma = ma.unwrap();

        Ok(Self {
            date,
            lunar_calendar: lunar,
            ju_shu,
            di_pan,
            tian_pan,
            shen_pan,
            star_pan,
            zhi_fu,
            zhi_shi,
            men_pan,
            yin_gan_pan,
            kong,
            ma,
        })
    }
}

// 乾：天（高，广阔，华丽，精美，震撼，荣誉第一，特性突显的人、事、物）
// 概念：有威严，傲慢，权力，战争，竞争，胆量，优胜，充实，满足，模范，正直，尊敬，喜悦，健壮，圆满，收获，
// 统帅，永久，创造，法则，本愿，高亢，核心，精华，向上，长辈，坚固，激烈
// 性情：好动少静，严正威武，重情讲义，威严豁达，正直勤勉，自尊高傲
// 形态：高档贵重，精致完美，高大雄伟，坚固圆滑，趾高气扬，白色，金黄色
// 天时：晴天，晴空，太阳，寒气，霜，雪，冰，雹，霰
// 地理：西北方，繁华地，首脑集中地，京都，大城市，型胜高亢之所，名胜古迹，
// 大会堂，广场，寺院，高级住宅，大厦，银行，警察局，机关，武馆，博物馆，
// 邮局，金属工厂，配件商店
// 人物：国家元首，主要领导人，寺院主持，总经理，老板，祖父，老者，名流，
// 专家，厂长，高贵的人，元老，恶人。
// 动物：狮子，大象，老虎，猪，熊，狗，马，鹅，鲸鱼，鹰，龙
// 植物：秋花，菊花，大树，能结果的树，药草
// 食物：水果，糖果，蛋糕，年羔，冰激凌，豆腐，鸡蛋，高级食品，火腿，香肠，干肉，马肉，米，麦，豆类，花生，䍶味食品
// 静物：金玉珠宝，高档用品，金钱，钟表，镜子，眼镜，武器，圆形金属，帽子，神佛物品，首饰，飞机，火车，轿车，自行车，摩托车，刀剑，头巾，面罩，高大物。
// 人体：头，颈，面部，肋骨，指甲，右腿，肺，大肠，皮毛，骨骼，男性生殖器，精液，右下腹，胸部
// 疾病：头痛，脑淤血，心脏病，肺部疾病，肋膜炎，必肿疾病，神经病，脖子扭伤，皮肤病，骨折，骨病，硬化性疾病，老病，旧病，伤寒，急性暴病，结肠疾病
// 时间：秋天，九十之交，戌、亥，年，月，日，时
// 色彩：大赤色，玄色，金黄色，白色，强烈的颜色
// 姓名排行：带金字旁者，在兄弟中排行老大，老四，老七

// 天干地支象意
// http://longyuqimen.cn/news/html/681.html

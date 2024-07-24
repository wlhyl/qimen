// import { Power, StarCategory, StarName, Vstar } from '../enum';

import { GanName } from '../enum/gan';

export interface QiMen {
  // 时间
  date: HoroDateTime;

  //  出生时刻的农历
  lunar_calendar: LunarCalendar;

  // 局数
  ju_shu: number;

  // 空亡
  kong: Array<string>;

  // 马星
  ma: string;

  // 地盘
  di_pan: Array<Array<Gan>>;
  // 天盘
  tian_pan: Array<Array<Gan>>;
  // 神盘
  shen_pan: Array<Shen>;

  // 值符
  zhi_fu: Star;
  // 星盘
  star_pan: Array<Star>;

  // 值使
  zhi_shi: Men;
  // 门盘
  men_pan: Array<Men>;
  // 隐干盘
  yin_gan_pan: Array<Array<Gan>>;

  // 地盘入墓
  di_pan_mu: Array<string>;
  // 天盘入墓
  tian_pan_mu: Array<string>;
  // 隐干盘墓
  yin_gan_pan_mu: Array<string>;
}

export interface HoroDateTime {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number; //时区
}

export interface LunarCalendar {
  // 闰年:true
  is_lean_year: boolean;

  // 农历年，干支表示
  lunar_year: string; // GanZhi,

  // 农历月，以正月、二月、......、十月、冬月、腊月表示
  lunar_month: string;

  // 农历日，以初一、初二、……、二十九、三十表示
  lunar_day: string;

  // 农历年干支，按节气换年
  lunar_year_gan_zhi: string; // GanZhi,

  // 农历月干支，按节气换月
  lunar_month_gan_zhi: string; // GanZhi,

  // 日干支
  lunar_day_gan_zhi: string; // GanZhi,

  // 时干支
  time_gan_zhi: string; // GanZhi,

  // 节
  solar_term_first: SolarTerm;

  // 中气
  solar_term_second: SolarTerm;
}

// 节气
export interface SolarTerm {
  name: string;
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
}

export interface QiMenElement {
  // 元素名
  name: string;
  // 描述
  describe: string;
}

export interface Gan extends QiMenElement {
  // 元素名
  name: GanName;
  // 入墓
  mu: boolean;
  // 描述
  describe: string;
}
export interface Shen extends QiMenElement {}
export interface Star extends QiMenElement {}
export interface Men extends QiMenElement {}

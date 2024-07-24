import { Injectable } from '@angular/core';
import { QiMenRequest } from 'src/app/interface/request-data';

@Injectable({
  providedIn: 'root',
})
export class QiMenStorageService {
  public set qiMenData(date: QiMenRequest) {
    localStorage.setItem('qimen_data', JSON.stringify(date));
  }

  public get qiMenData(): QiMenRequest {
    let j = localStorage.getItem('qimen_data');
    if (j) {
      return JSON.parse(j) as QiMenRequest;
    }

    let t = this.nowDate();

    return {
      year: t.year,
      month: t.month,
      day: t.day,
      hour: t.hour,
      minute: t.minute,
      second: t.second,
      describe: '',
    };
  }

  constructor() {}

  private nowDate(): date {
    let t = new Date();
    let year = t.getFullYear();
    let month = t.getMonth() + 1;
    let day = t.getDate();
    let hour = t.getHours();
    let minute = t.getMinutes();
    let second = t.getSeconds();

    let st = false;

    // 判断夏令时
    let d1 = new Date(year, 1, 1);
    let tz = d1.getTimezoneOffset() / -60;
    // let d2 = new Date(this.horo.year,7,1);
    if (t.getTimezoneOffset() != d1.getTimezoneOffset()) {
      st = true;
      tz -= 1;
    }

    return {
      year,
      month,
      day,
      hour,
      minute,
      second,
      tz,
      st,
    };
  }
}

interface date {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;
}

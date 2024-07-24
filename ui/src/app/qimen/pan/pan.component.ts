import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Platform } from '@ionic/angular';
import { GuaName } from 'src/app/enum/gua';
import { QiMen } from 'src/app/interface/response';
import { ApiService } from 'src/app/services/api/api.service';
import { HouseClickService } from 'src/app/services/house-click/house-click.service';
import { QiMenStorageService } from 'src/app/services/qimen_storage/qiemn_storage.service';

@Component({
  selector: 'app-pan',
  templateUrl: './pan.component.html',
  styleUrls: ['./pan.component.scss'],
})
export class PanComponent implements OnInit {
  title = '阴盘奇门';

  qiMenData = this.storage.qiMenData;

  pan: QiMen | null = null;
  men: any;

  get ju_shu() {
    if (this.pan === null) return null;
    let s = '阳遁';
    if (this.pan.ju_shu < 0) s = '阴遁';
    switch (Math.abs(this.pan.ju_shu)) {
      case 1:
        return `${s}一局`;
      case 2:
        return `${s}二局`;
      case 3:
        return `${s}三局`;
      case 4:
        return `${s}四局`;
      case 5:
        return `${s}五局`;
      case 6:
        return `${s}六局`;
      case 7:
        return `${s}七局`;
      case 8:
        return `${s}八局`;
      default:
        return `${s}九局`;
    }
  }

  readonly gong_names = [
    GuaName.坎,
    GuaName.艮,
    GuaName.震,
    GuaName.巽,
    GuaName.离,
    GuaName.坤,
    GuaName.兑,
    GuaName.乾,
  ];

  is_kong(guaName: GuaName) {
    if (this.pan == null) return false;

    switch (guaName) {
      case GuaName.坎:
        return this.pan.kong.includes('子');
      case GuaName.艮:
        return this.pan.kong.includes('丑') || this.pan.kong.includes('寅');
      case GuaName.震:
        return this.pan.kong.includes('卯');
      case GuaName.巽:
        return this.pan.kong.includes('辰') || this.pan.kong.includes('巳');
      case GuaName.离:
        return this.pan.kong.includes('午');
      case GuaName.坤:
        return this.pan.kong.includes('未') || this.pan.kong.includes('申');
      case GuaName.兑:
        return this.pan.kong.includes('酉');
      case GuaName.乾:
        return this.pan.kong.includes('戌') || this.pan.kong.includes('亥');
    }
  }

  is_ma(guaName: GuaName) {
    if (this.pan == null) return false;

    switch (guaName) {
      case GuaName.艮:
        return this.pan.ma == '寅';
      case GuaName.巽:
        return this.pan.ma == '巳';
      case GuaName.坤:
        return this.pan.ma == '申';
      case GuaName.乾:
        return this.pan.ma == '亥';
      default:
        return false;
    }
  }

  width = 400;
  height = this.width;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  loading = false;

  constructor(
    private api: ApiService,
    private storage: QiMenStorageService,
    public houseClickService: HouseClickService,
    private titleService: Title,
    private platform: Platform
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
    this.getZiweiPan();

    this.platform.ready().then(() => {
      let width = this.platform.width();
      if (this.width > width) {
        this.width = width - 20;
        this.height = width - 20;
      }
    });
  }

  getZiweiPan() {
    this.loading = true;
    this.api.getQiMen(this.qiMenData).subscribe({
      next: (response) => (this.pan = response),
      error: (error) => {
        const message = error.message + ' ' + error.error.message;
        this.message = message;
        this.isAlertOpen = true;
      },
      complete: () => (this.loading = false),
    });
  }

  changeStep(step: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  }) {
    let date = new Date(
      this.qiMenData.year,
      this.qiMenData.month - 1,
      this.qiMenData.day,
      this.qiMenData.hour,
      this.qiMenData.minute,
      this.qiMenData.second
    );

    date.setFullYear(date.getFullYear() + step.year);
    date.setMonth(date.getMonth() + step.month);
    date.setDate(date.getDate() + step.day);
    date.setHours(date.getHours() + step.hour);
    date.setMinutes(date.getMinutes() + step.minute);
    date.setSeconds(date.getSeconds() + step.second);

    this.qiMenData.year = date.getFullYear();
    this.qiMenData.month = date.getMonth() + 1;
    this.qiMenData.day = date.getDate();
    this.qiMenData.hour = date.getHours();
    this.qiMenData.minute = date.getMinutes();
    this.qiMenData.second = date.getSeconds();

    this.getZiweiPan();
  }
}

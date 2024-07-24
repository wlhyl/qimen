import { Component, Input } from '@angular/core';
import { GanName } from 'src/app/enum/gan';
import { GuaName, GuaNum } from 'src/app/enum/gua';
import { MenName } from 'src/app/enum/men';
import { Gan, Men, QiMenElement, Shen, Star } from 'src/app/interface/response';
import { HouseClickService } from 'src/app/services/house-click/house-click.service';

@Component({
  selector: 'app-house',
  templateUrl: './house.component.html',
  styleUrls: ['./house.component.scss'],
})
export class HouseComponent {
  @Input()
  kong = false;
  @Input()
  ma = false;
  @Input()
  name = GuaName.坎;
  @Input()
  di: Array<Gan> = [];
  @Input()
  tian: Array<Gan> = [];
  @Input()
  shen: Shen = {
    name: '',
    describe: '',
  };

  @Input()
  star: Star = {
    name: '',
    describe: '',
  };

  @Input()
  men: Men = {
    name: '',
    describe: '',
  };

  @Input()
  yin_gan: Array<Gan> = [];

  get gong_num() {
    return GuaNum(this.name);
  }

  get is_po() {
    switch (this.name) {
      case GuaName.坎:
        if (this.men.name == MenName.生 || this.men.name == MenName.死)
          return true;
        else return false;
      case GuaName.艮:
      case GuaName.坤:
        if (this.men.name == MenName.伤 || this.men.name == MenName.杜)
          return true;
        else return false;
      case GuaName.震:
      case GuaName.巽:
        if (this.men.name == MenName.惊 || this.men.name == MenName.开)
          return true;
        else return false;
      case GuaName.离:
        if (this.men.name == MenName.休) return true;
        else return false;

      case GuaName.兑:
      case GuaName.乾:
        if (this.men.name == MenName.景) return true;
        else return false;
    }
  }

  constructor(private houseClickService: HouseClickService) {}

  showDescribe(element: QiMenElement) {
    this.houseClickService.describe = `${element.name}：\n${element.describe}`;
  }

  is_xing(gan: GanName) {
    switch (gan) {
      case GanName.戊:
        if (this.name == GuaName.震) return true;
        else return false;
      case GanName.癸:
        if (this.name == GuaName.巽) return true;
        else return false;
      case GanName.壬:
        if (this.name == GuaName.巽) return true;
        else return false;
      case GanName.辛:
        if (this.name == GuaName.离) return true;
        else return false;
      case GanName.庚:
        if (this.name == GuaName.艮) return true;
        else return false;
      case GanName.己:
        if (this.name == GuaName.坤) return true;
        else return false;
      default:
        return false;
    }
  }
}

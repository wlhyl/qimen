import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { HoroCommonModule } from '../horo-common/horo-common.module';

import { QimenPageRoutingModule } from './qimen-routing.module';

import { QimenPage } from './qimen.page';
import { PanComponent } from './pan/pan.component';
import { HouseComponent } from './house/house.component';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    QimenPageRoutingModule,
    HoroCommonModule,
  ],
  declarations: [QimenPage, PanComponent, HouseComponent],
})
export class QimenPageModule {}

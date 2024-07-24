import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { QimenPage } from './qimen.page';
import { PanComponent } from './pan/pan.component';

const routes: Routes = [
  {
    path: '',
    component: QimenPage
  },
  {
    path: 'pan',
    component: PanComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class QimenPageRoutingModule {}

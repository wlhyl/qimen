import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { QiMenStorageService } from '../services/qimen_storage/qiemn_storage.service';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-qimen',
  templateUrl: './qimen.page.html',
  styleUrls: ['./qimen.page.scss'],
})
export class QimenPage implements OnInit {
  title = '阴盘奇门';

  qiMenData = this.storage.qiMenData;

  constructor(
    private storage: QiMenStorageService,
    private router: Router,
    private route: ActivatedRoute,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }


  getQiMen(){
    this.storage.qiMenData= this.qiMenData;
    this.router.navigate(['pan'], {
      relativeTo: this.route,
    });
  }
}

import { ComponentFixture, TestBed } from '@angular/core/testing';
import { QimenPage } from './qimen.page';

describe('QimenPage', () => {
  let component: QimenPage;
  let fixture: ComponentFixture<QimenPage>;

  beforeEach(() => {
    fixture = TestBed.createComponent(QimenPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

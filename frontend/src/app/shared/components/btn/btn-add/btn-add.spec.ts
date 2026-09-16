import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BtnAdd } from './btn-add';

describe('BtnAdd', () => {
  let component: BtnAdd;
  let fixture: ComponentFixture<BtnAdd>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BtnAdd]
    })
      .compileComponents();

    fixture = TestBed.createComponent(BtnAdd);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

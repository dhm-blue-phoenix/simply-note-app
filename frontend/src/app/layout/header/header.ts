import {Component} from '@angular/core';

import {BtnHeader, BtnTypes} from '../../shared/components/btn/btn-header/btn-header';

@Component({
    imports: [BtnHeader],
    selector: 'app-header',
    styleUrl: './header.css',
    templateUrl: './header.html',
})
export class Header {
    public set_btn(type: BtnTypes): BtnTypes {
        return type;
    }
}

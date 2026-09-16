import {Component, inject} from '@angular/core';

import {Backend} from '../../shared/services/backend';

@Component({
    imports: [],
    selector: 'app-main',
    styleUrl: './main.css',
    templateUrl: './main.html',
})
export class Main {
    private backend = inject(Backend);
    public notes = [];
}

import { Service } from '@angular/core';
import { environment } from '../../enviroment/enviroment';

type NoteStatus = "notes" | "trash";
type Notes = Note[];
interface Note {
    id: string,
    title: String,
    content: String,
    status: NoteStatus,
    created_at: String,
}

type Endpoint = "notes" | "note";

interface BackendResult {
    is_ok: boolean,
    status: "error" | number,
    content: unknown
}

@Service()
export class Backend {
    private readonly BACKEND_URLS: RequestInfo = environment.BACKEND_URL + "/api/";

    constructor() {
        this.testing();
    }

    /*
    * Hierbei handelt es sich um eine Test funktion um die Backend API
    * beim Implementiren gleich Testen zu können!
    * */
    private async testing(): Promise<void> {
        const data: BackendResult = await this.get_data('notes');
        const data2: BackendResult = await this.get_data('note');
        console.log(data);
        console.log(data2);
    }

    private async get_data(endpoint: Endpoint): Promise<BackendResult> {
        const url: RequestInfo = `${this.BACKEND_URLS}${endpoint}`;
        try {
            const resp: Response = await fetch(url, {
                method: 'GET'
            });
            const result: Notes = await resp.json();
            return {
                is_ok: resp.ok,
                status: resp.status,
                content: result
            };
        } catch (error) {
            return {
                is_ok: false,
                status: "error",
                content: error
            };
        }
    }
}

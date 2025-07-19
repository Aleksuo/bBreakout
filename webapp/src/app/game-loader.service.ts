import { Injectable } from '@angular/core';
import initWasm, * as game from '../wasm/bbreakout';

@Injectable({ providedIn: 'root' })
export class GameLoaderService {
  private modulePromise = (async () => {
    await initWasm(new URL('bbreakout_bg.wasm', import.meta.url));
    return game;
  })();

  ready() { return this.modulePromise; }
}
import { Component, inject } from '@angular/core';
import { GameLoaderService } from './game-loader.service.js';

@Component({
  selector: 'app-root',
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App  {
  gameLoader = inject(GameLoaderService)

  isReady$ = this.gameLoader.ready();

}

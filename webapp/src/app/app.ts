import { Component, inject, OnInit, signal } from '@angular/core';
import { GameLoaderService } from './game-loader.service.js';
import { AsyncPipe } from '@angular/common';

@Component({
  selector: 'app-root',
  templateUrl: './app.html',
  styleUrl: './app.scss',
  imports: [
    AsyncPipe
  ]
})
export class App  {
  gameLoader = inject(GameLoaderService)

  isReady$ = this.gameLoader.ready();

}

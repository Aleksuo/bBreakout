import { cp } from 'node:fs/promises';
import { resolve } from 'node:path';

await cp(
  resolve('bevy-breakout/assets'),
  resolve('webapp/src/assets/'),
  { recursive: true, force: true }
);
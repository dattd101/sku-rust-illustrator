import { cpSync, mkdirSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { join } from 'node:path';
const root = process.cwd();
mkdirSync(join(root, 'dist'), { recursive: true });
cpSync(join(root, 'src/index.html'), join(root, 'dist/index.html'));
cpSync(join(root, 'src/app.js'), join(root, 'dist/app.js'));
const bin = join(root, 'node_modules/.bin/tailwindcss' + (process.platform === 'win32' ? '.cmd' : ''));
execFileSync(bin, ['-i','src/input.css','-o','dist/styles.css','--minify'], { cwd: root, stdio:'inherit', shell:process.platform==='win32' });

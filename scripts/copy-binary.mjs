import { copyFile, mkdir } from 'node:fs/promises';
const root = new URL('../', import.meta.url);
await mkdir(new URL('dist/bin/', root), { recursive: true });
await copyFile(new URL('target/release/restore-drill', root), new URL('dist/bin/restore-drill-linux-x86_64', root));

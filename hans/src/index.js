import fs from 'fs';
import path from 'path';

async function main(argv) {
  const day = parseInt(argv[2]);
  try {
    const moduleName = `day${String(day).padStart(2, '0')}`;
    const module = await import(`./${moduleName}/index.js`);

    const inputFile = fs.readFileSync(path.resolve('src', moduleName, 'input'));
    const input = inputFile.toString();

    const result = module.resolve(input);
    console.log(result);
  } catch (e) {
    console.error(e);
  }
}

main(process.argv);

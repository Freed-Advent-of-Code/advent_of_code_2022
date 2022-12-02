import fs from 'fs';
import path from 'path';

async function main(argv) {
  const day = parseInt(argv[2]);

  const moduleName = `day${String(day).padStart(2, '0')}`;
  const module = await import(`./${moduleName}/index.js`);

  const inputFile = fs.readFileSync(path.resolve('src', moduleName, 'input'));
  const input = inputFile.toString();

  const startTime = new Date();
  const result = module.resolve(input);
  console.log('Puzzle answer is', result);
  console.log('It takes', `${new Date() - startTime}ms`);
}

main(process.argv);

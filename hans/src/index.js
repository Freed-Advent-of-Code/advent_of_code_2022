async function main(argv) {
  const day = parseInt(argv[2]);
  const moduleName = `day${String(day).padStart(2, '0')}`;
  const module = await import(`./${moduleName}/index.js`);

  try {
    const result = module.resolve();
    console.log(result);
  } catch (e) {}
}

main(process.argv);

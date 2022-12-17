const dayText: string[] = ["day01", "day02", "day03"];

async function index() {
  const argv = process.argv[2];
  const day = dayText[parseInt(argv) - 1];

  const run = await import(`./${day}/index.ts`);

  console.log(run.answer(day));
}

index();

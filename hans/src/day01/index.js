export function resolve(input) {
  return [resolvePuzzle1(input), resolvePuzzle2(input)];
}

function resolvePuzzle1(input) {
  return Math.max(
    ...input
      .split('\n\n')
      .map((elf) =>
        elf.split('\n').reduce((total, curr) => total + Number(curr), 0),
      ),
  );
}

function resolvePuzzle2(input) {
  return input
    .split('\n\n')
    .map((elf) =>
      elf.split('\n').reduce((total, curr) => total + Number(curr), 0),
    )
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((total, curr) => total + curr, 0);
}

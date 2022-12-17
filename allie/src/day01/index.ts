import fs from "fs";

export function answer(day: string): { part1: number; part2: number } {
  const inputFile = fs.readFileSync(`src/input/${day}.txt`);
  const input = inputFile.toString().trim();

  return { part1: part1(input), part2: part2(input) };
}

function part1(input: string) {
  const totalCalories: number[] = input
    .split("\n\n")
    .map((cal) => cal.split("\n").reduce((acc, cur) => acc + Number(cur), 0));

  return Math.max(...totalCalories);
}

function part2(input: string) {
  const totalCalories: number[] = input
    .split("\n\n")
    .map((cal) => cal.split("\n").reduce((acc, cur) => acc + Number(cur), 0));

  return totalCalories
    .sort((a, b) => a - b)
    .slice(-3)
    .reduce((acc, cur) => acc + Number(cur), 0);
}

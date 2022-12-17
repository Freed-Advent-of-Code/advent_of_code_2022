const { assert } = require("console");
const fs = require("fs");

main();

function main() {
  const part1Result = part1("day13.txt");
  console.log("p1 result:", part1Result);

  const part2Result = part2("day13.txt");
  console.log("p2 result:", part2Result);
}

function part1(filename) {
  try {
    const data = fs.readFileSync(filename);
    let result = 0;
    data
      .toString()
      .split("\n\n")
      .forEach((lines, index) => {
        const [first, second] = lines.split("\n").map((a) => eval(a));

        if (compare(first, second) < 0) {
          result += index + 1;
        }
      });
    return result;
  } catch (err) {
    console.error(err);
  }
}

function part2(filename) {
  try {
    const data = fs.readFileSync(filename);

    const d1 = [[2]];
    const d2 = [[6]];

    const arrays = data
      .toString()
      .split("\n")
      .filter((arr) => arr !== "")
      .map((arrStr) => eval(arrStr));

    const concatArray = [d1, d2, ...arrays];
    concatArray.sort((first, second) => compare(first, second));

    const i1 = concatArray.findIndex((arr) => arr === d1) + 1;
    const i2 = concatArray.findIndex((arr) => arr === d2) + 1;

    console.log(i1, i2);

    return i1 * i2;
  } catch (err) {
    console.error(err);
  }
}

function test_part1() {
  const result = part1("day13-test.txt");
  console.log(result);
  assert(result === 13);
}

function test_part2() {
  const result = part2("day13-test.txt");
  console.log(result);
}

/**
 * Compares two entities and returns -1, 0, or 1
 * @param {Array or Number} first
 * @param {Array or Number} second
 *
 * @returns -1 if first < second, 0 if first == second, 1 if first > second
 */
function compare(first, second) {
  // Array vs Array
  if (Array.isArray(first) && Array.isArray(second)) {
    const minLength = Math.min(first.length, second.length);
    for (let i = 0; i < minLength; i++) {
      const comparison = compare(first[i], second[i]);
      switch (comparison) {
        case 1:
          return 1;
        case -1:
          return -1;
        default:
          continue;
      }
    }
    if (first.length > second.length) {
      return 1;
    }
    if (first.length === second.length) {
      return 0;
    }
    return -1;
  }

  // Number vs Array
  if (typeof first === "number" && Array.isArray(second)) {
    return compare([first], second);
  }

  if (Array.isArray(first) && typeof second === "number") {
    return compare(first, [second]);
  }

  // Number vs Number
  if (first > second) return 1;
  if (first < second) return -1;
  return 0;
}

import readline from "readline/promises";
const rl = readline.createInterface(process.stdin);

/** @type {(any, any) => number[]} */
const reduce_fn = (prev, next) =>
  (Array.isArray(prev) ? prev : [prev]).flatMap((a) => [
    a + next,
    a * next,
    parseInt(a.toString() + next.toString()),
  ]);

let answer = 0;

for await (const line of rl) {
  let cursor = 0;
  const expected_result = parseInt(
    line.slice(cursor, (cursor = line.indexOf(":")))
  );

  cursor += 2; // consume space
  const numbers = line
    .substring(cursor)
    .split(/\s+/g)
    .map((str) => parseInt(str));

  /** @type {number[]} */
  const results = numbers.reduce(reduce_fn);

  if (results.includes(expected_result)) {
    answer += expected_result;
  }
}

console.log("Answer: %d", answer);

/**
 * Advent of Code 2023
 * Day 08
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import fs from "fs";
// Function to solve part 1 of the challenge
export const solvePart2 = () => {
  fs.readFile("input2.txt", "utf-8", (err, data) => {
    const [directions, ...nodes] = data.split("\n");

    const nodesMap = nodes.reduce((acc, x) => {
      const self = x.substring(0, 3);
      const L = x.substring(7, 10);
      const R = x.substring(12, 15);
      acc = { ...acc, [self]: { L, R } };
      return acc;
    }, {});

    let currentNodes = Object.keys(nodesMap).filter((node) =>
      node.endsWith("A"),
    );

    const pathLengths = [];

    for (let i = 0; i < currentNodes.length; i++) {
      let dirIdx = 0;
      let steps = 0;
      let node = currentNodes[i];
      while (!node.endsWith("Z")) {
        dirIdx = dirIdx % directions.length;
        node = nodesMap[node][directions[dirIdx]];
        steps++;
        dirIdx++;
      }
      pathLengths.push(steps);
    }
    console.log(pathLengths.reduce((acc, curr) => lcm(acc, curr), 1));
  });
};

solvePart2();

// Euclid algorithm for Greates Common Divisor
function gcd(a, b) {
  return !b ? a : gcd(b, a % b);
}

// Least Common Multiple function
function lcm(a, b) {
  return a * (b / gcd(a, b));
}

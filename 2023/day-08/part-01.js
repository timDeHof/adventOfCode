/**
 * Advent of Code 2023
 * Day 08
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import fs from "fs";
// Function to solve part 1 of the challenge
export const solvePart1 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    const [directions, ...nodes] = data.split("\n");

    const nodesMap = nodes.reduce((acc, x) => {
      const self = x.substring(0, 3);
      const L = x.substring(7, 10);
      const R = x.substring(12, 15);
      acc = { ...acc, [self]: { L, R } };
      return acc;
    }, {});

    let dirIdx = 0;
    let steps = 0;
    let current = "AAA";

    while (current != "ZZZ") {
      dirIdx = dirIdx % directions.length;
      current = nodesMap[current][directions[dirIdx]];
      steps++;
      dirIdx++;
    }
    console.log(steps);
  });
};
//# sourceMappingURL=dayTemplate.js.map
solvePart1();

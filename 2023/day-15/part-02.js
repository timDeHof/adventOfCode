/**
 * Advent of Code 2023
 * Day 15
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
import fs from "fs";

/**
 * Calculate the hash of a string
 * @param {string} str - The input string
 * @returns {number} - The calculated hash
 */
function calculateHash(str) {
  return str.split("").reduce((acc, s) => {
    acc += s.charCodeAt(0);
    acc *= 17;
    acc %= 256;
    return acc;
  }, 0);
}

/**
 * Calculate the sum of powers.
 * @param {Array} boxes
 * @returns {number}
 */
function calculatePowers(boxes) {
  return boxes
    .flatMap(function (box, boxIdx) {
      return box.map(function (lens, lensIdx) {
        return (1 + boxIdx) * (1 + lensIdx) * lens.focalLength;
      });
    })
    .reduce((acc, h) => acc + h, 0);
}

fs.readFile("input.txt", "utf-8", (err, data) => {
  const commands = data.split(",");
  const boxes = new Array(256).fill("").map(() => []);

  for (const command of commands) {
    if (command.endsWith("-")) {
      const label = command.slice(0, -1);
      const hash = calculateHash(label);
      boxes[hash] = boxes[hash].filter((lens) => lens.label !== label);
    } else {
      const [label, f] = command.split("=");
      const focalLength = Number(f);
      const hash = calculateHash(label);
      const index = boxes[hash].findIndex((lens) => lens.label === label);
      if (index === -1) {
        boxes[hash].push({ label, focalLength });
      } else {
        boxes[hash][index].focalLength = focalLength;
      }
    }
  }

  const powers = calculatePowers(boxes);
  console.log(powers);
});

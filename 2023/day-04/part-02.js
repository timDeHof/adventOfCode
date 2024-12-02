/**
 * Advent of Code 2023
 * Day 04
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import readFile from "./utils.js";
import fs from "fs";
// Function to solve part 1 of the challenge

fs.readFile("input.txt", "utf-8", (err, data) => {
  const input = data.split("\n").map((x) => x.replace(/  /g, " 0"));
  const cardCount = new Array(input.length).fill(1);

  input.forEach((row, index) => {
    const [, cards] = row.split(": ");
    const [winners, myCards] = cards.split(" | ");

    const point = myCards
      .split(" ")
      .filter((card) => winners.includes(card)).length;

    if (point) {
      for (let i = index + 1; i < index + 1 + point; i++) {
        if (cardCount[i]) cardCount[i] += cardCount[index] || 0;
      }
    }
  });

  console.log(cardCount.reduce((acc, cv) => acc + cv, 0));
});

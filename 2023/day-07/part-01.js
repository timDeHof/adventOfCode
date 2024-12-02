/**
 * Advent of Code 2023
 * Day 07
 *
 * URL: https://adventofcode.com/2023/day/7
 */
// Import necessary libraries or modules
import fs from "fs";
// Function to solve part 1 of the challenge

const cardValues = {
  T: 10,
  J: 11,
  Q: 12,
  K: 13,
  A: 14,
};
export const solvePart1 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    const lines = data.split("\n");
    lines.sort((a, b) => {
      const handA = a.split(" ")[0];
      const handB = b.split(" ")[0];
      return sortHands(handB, handA);
    });
    const wins = lines.reduce((acc, line, i) => {
      const bid = line.split(" ")[1];
      acc += bid * (i + 1);
      return acc;
    }, 0);

    console.log(wins);
  });
};
//# sourceMappingURL=dayTemplate.js.map
solvePart1();

const sortHands = (handA, handB) => {
  const handValueA = getRank(handA);
  const handValueB = getRank(handB);
  if (handValueA > handValueB) return -1;
  if (handValueB > handValueA) return 1;
  return tieBreaker(handA, handB);
};

const getRank = (hand) => {
  const handSet = getHandSet(hand);
  let hasPair = false;
  let hasThree = false;
  for (let key in handSet) {
    const val = handSet[key];
    if (val === 5) {
      return 7;
    }
    if (val === 4) {
      return 6;
    }
    if ((val === 3 && hasPair) || (val === 2 && hasThree)) {
      return 5;
    }
    if (val === 2 && hasPair) {
      return 3;
    }
    if (val === 3) {
      hasThree = true;
    }
    if (val === 2) {
      hasPair = true;
    }
  }
  if (hasThree) return 4;
  if (hasPair) return 2;
  return 1;
};

const getHandSet = (hand) => {
  const set = {};
  hand.split("").forEach((element) => {
    if (set[element]) {
      set[element] += 1;
    } else {
      set[element] = 1;
    }
  });
  return set;
};

const tieBreaker = (handA, handB) => {
  for (let i = 0; i < handA.length; i++) {
    const cardA = handA[i];
    const cardB = handB[i];
    const handAVal = isNaN(Number(cardA)) ? cardValues[cardA] : Number(cardA);
    const handBVal = isNaN(Number(cardB)) ? cardValues[cardB] : Number(cardB);
    if (handAVal > handBVal) return -1;
    if (handBVal > handAVal) return 1;
  }
  return 0;
};

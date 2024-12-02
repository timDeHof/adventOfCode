import readFile from "./parseFile.js";

const findFirstNumber = (line) => {
  for (let i = 0; i < line.length; i++) {
    const currentChar = line[i];

    if (Number(currentChar)) {
      return currentChar;
    }

    const combine = line.slice(0, i + 1);

    for (const number in NumberKey) {
      if (combine.includes(number)) {
        return NumberKey[number];
      }
    }
  }
};

const findLastNumber = (line) => {
  for (let i = line.length - 1; i >= 0; i--) {
    const currentChar = line[i];

    if (Number(currentChar)) {
      return currentChar;
    }

    const combine = line.slice(i, i + line.length);

    for (const number in NumberKey) {
      if (combine.includes(number)) {
        return NumberKey[number];
      }
    }
  }
};
const NumberKey = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
};

export const Part02 = async () => {
  const data = await readFile("input2.txt");

  let total = 0;

  data.forEach((line) => {
    const firstNumber = findFirstNumber(line);
    const lastNumber = findLastNumber(line);
    const digit = Number(firstNumber + lastNumber);
    total += digit;
  });
  return total;
};

console.log(await Part02());

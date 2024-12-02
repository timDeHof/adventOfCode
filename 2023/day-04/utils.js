import { promises as fs } from "fs";

async function readFile(file) {
  try {
    const data = await fs.readFile(file, "utf-8");
    return data.split("\n");
  } catch (error) {
    console.error("Error reading file:", error.message);
    throw error;
    return [];
  }
}

export default readFile;

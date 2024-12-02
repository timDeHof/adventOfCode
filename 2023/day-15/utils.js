var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import { promises as fs } from "fs";
function readFile(file) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const data = yield fs.readFile(file, "utf-8");
            return data.split("\n");
        }
        catch (error) {
            if (error instanceof Error && "code" in error) {
                if (error.code === "EEXIST") {
                    console.error(`parse file already exists.`);
                }
                else {
                    console.error(`An error occurred while creating parse file:`, error);
                }
                console.error("Error reading file:", error.message);
                throw error;
            }
        }
    });
}
export default readFile;
//# sourceMappingURL=parseTemplate.js.map
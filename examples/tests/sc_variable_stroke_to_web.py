import json
import os

DIRECTORY = "./variable_stroke"
OUTPUT_FILE = "../../web_tests/variable_stroke_tests.json"

all_data = []

for filename in sorted(os.listdir(DIRECTORY)):
    if filename.endswith(".json"):
        file_path = os.path.join(DIRECTORY, filename)

        with open(file_path, "r", encoding="utf-8") as file:
            try:
                data = json.load(file)
                if "stroke" in data:
                    all_data.append(
                        {
                            "stroke": data.get("stroke"),
                            "scale": data.get("scale"),
                        }
                    )
                else:
                    print(f"Skipping incomplete file: {filename}")
            except json.JSONDecodeError as error:
                print(f"Skipping invalid JSON: {filename} ({error})")

with open(os.path.join(DIRECTORY, OUTPUT_FILE), "w", encoding="utf-8") as file:
    json.dump(all_data, file, indent=4)

print(f"Aggregated {len(all_data)} JSON files into {OUTPUT_FILE}")

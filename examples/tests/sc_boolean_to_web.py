import os
import json

# Path to the folder with cleaned JSON files
DIRECTORY = "./boolean"
OUTPUT_FILE = "../../web_tests/boolean_tests.json"

all_data = []

# Iterate through all JSON files in the folder
for filename in sorted(os.listdir(DIRECTORY)):
    if filename.endswith(".json"):
        file_path = os.path.join(DIRECTORY, filename)

        with open(file_path, 'r', encoding='utf-8') as f:
            try:
                data = json.load(f)
                if "subjPaths" in data and "clipPaths" in data:
                    all_data.append(data)
                else:
                    print(f"Skipping incomplete file: {filename}")
            except json.JSONDecodeError as e:
                print(f"Skipping invalid JSON: {filename} ({e})")

# Write the aggregated result to a new JSON file
with open(os.path.join(DIRECTORY, OUTPUT_FILE), 'w', encoding='utf-8') as f:
    json.dump(all_data, f, indent=4)

print(f"Aggregated {len(all_data)} JSON files into {OUTPUT_FILE}")

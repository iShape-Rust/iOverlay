import os
import json

# Hardcoded directory path where your JSON files are located
DIRECTORY = "./boolean"

# Loop through each file in the directory
for filename in os.listdir(DIRECTORY):
    if filename.endswith(".json"):
        file_path = os.path.join(DIRECTORY, filename)

        with open(file_path, 'r', encoding='utf-8') as f:
            try:
                data = json.load(f)
            except json.JSONDecodeError as e:
                print(f"Skipping invalid JSON: {filename} ({e})")
                continue

        # Keep only subjPaths and clipPaths
        new_data = {
            "subjPaths": data.get("subjPaths"),
            "clipPaths": data.get("clipPaths")
        }

        # Write the cleaned JSON back to the file
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(new_data, f, indent=4)

print("Finished updating all JSON files.")

import os
import json


input_folder = "./tests/stroke"
output_file = "./tests_stroke.json"

def merge_json_files(input_folder, output_file):
    combined_data = []

    for index in range(4):
        filename = f"test_{index}.json"
        filepath = os.path.join(input_folder, filename)

        if os.path.exists(filepath):  # Check if the file exists
            try:
                # Open and parse the JSON file
                with open(filepath, "r") as file:
                    data = json.load(file)

                # Extract only the required properties
                entry = {
                    "scale": data.get("scale", 1000),
                    "stroke": data.get("stroke", [])
                }

                # Add to the combined data list
                combined_data.append(entry)
            except Exception as e:
                print(f"Error processing file {filename}: {e}")
        else:
            print(f"File {filename} does not exist. Skipping.")

    try:
        with open(output_file, "w") as outfile:
            json.dump(combined_data, outfile, indent=2)
        print(f"Successfully written to {output_file}")
    except Exception as e:
        print(f"Error writing output file: {e}")

# Run the function
merge_json_files(input_folder, output_file)

#!/usr/bin/env python3
import os
import json
from datetime import datetime, timezone

def merge_benchmarks(benchmark_dir):
    out = {
        'meta': {
            'lastUpdated': datetime.now(timezone.utc).isoformat(),
        },
    }
    combined = {}
    
    for subdir, _, files in os.walk(benchmark_dir):
        if files:
            # Extracting category from directory name, e.g., "16-shared" or "metal"
            category = os.path.basename(subdir)
            
            for file in files:
                if file.endswith('.json'):  # Make sure it's a JSON file
                    # Reading individual benchmark file
                    filepath = os.path.join(subdir, file)
                    with open(filepath, 'r') as f:
                        try:
                            data = json.load(f)
                            # Handle meta json differently
                            if file == 'meta.json':
                                out['meta'] = data
                                continue

                            if 'results' in data:  # Make sure 'results' key exists
                                benchmark_name = os.path.splitext(file)[0]  # Removing .json extension to get benchmark name
                                
                                # Adding data to the combined dictionary
                                if benchmark_name not in combined:
                                    combined[benchmark_name] = {}
                                
                                results = data['results']
                                results_map = {item["name"]: item for item in results}

                                combined[benchmark_name][category] = results_map
                            else:
                                print(f"Warning: 'results' key not found in {filepath}")
                        except json.JSONDecodeError:
                            print(f"Warning: Could not decode JSON in {filepath}")
                
    out['frameworks'] = combined
    return out

if __name__ == "__main__":
    benchmark_dir = ".benchmarks"
    
    # Merging benchmarks
    combined_data = merge_benchmarks(benchmark_dir)
    
    # Writing the combined data to combined.json
    with open("benchmarks.json", 'w') as f:
        json.dump(combined_data, f, indent=4)
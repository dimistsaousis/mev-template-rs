import json
import argparse

def compare_json(json1, json2):
    if type(json1) != type(json2):
        return False

    if isinstance(json1, dict):
        if set(json1.keys()) != set(json2.keys()):
            return False
        return all(compare_json(json1[key], json2[key]) for key in json1)

    if isinstance(json1, list):
        if len(json1) != len(json2):
            return False
        json1_sorted = sorted(json1, key=str)
        json2_sorted = sorted(json2, key=str)
        return all(compare_json(item1, item2) for item1, item2 in zip(json1_sorted, json2_sorted))

    return json1 == json2

def main():
    parser = argparse.ArgumentParser(description='Compare two JSON files.')
    parser.add_argument('file1', type=str, help='First JSON file.')
    parser.add_argument('file2', type=str, help='Second JSON file.')

    args = parser.parse_args()
    with open(args.file1, 'r') as f1, open(args.file2, 'r') as f2:
        json1 = json.load(f1)
        json2 = json.load(f2)
    print("EQUAL" if compare_json(json1, json2) else "NOT EQUAL")

if __name__ == "__main__":
    main()

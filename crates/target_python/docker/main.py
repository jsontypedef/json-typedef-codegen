import json
import sys
from gen import MAIN

for line in sys.stdin:
    value = MAIN.from_json_data(json.loads(line))
    print(json.dumps(value.to_json_data()))

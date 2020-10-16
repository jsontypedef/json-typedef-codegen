import json
import sys

from CODEGEN_DIR import MAIN_CLASS

for line in sys.stdin:
    value = MAIN_CLASS.from_json(json.loads(line))
    print(json.dumps(value.to_json()))

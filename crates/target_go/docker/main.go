package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"example.com/jtd_codegen_e2e"
)

func main() {
	decoder := json.NewDecoder(os.Stdin)
	i := 0
	for {
		var input jtd_codegen_e2e.MAIN
		if err := decoder.Decode(&input); err != nil {
			if err == io.EOF {
				return
			}

			panic(fmt.Errorf("%d: %w", i, err))
		}

		out, err := json.Marshal(input)
		if err != nil {
			panic(fmt.Errorf("%d: %w", i, err))
		}

		fmt.Println(string(out))
		i++
	}
}

package main

import (
	"flag"
	"fmt"
	"os"
)

type Config struct {
	From string
	To   string
}

func parse() Config {
	var from string
	var to string

	flag.StringVar(&from, "from", "", "source save file")
	flag.StringVar(&from, "source", "", "source save file (alias)")
	flag.StringVar(&to, "to", "", "target save file")
	flag.StringVar(&to, "target", "", "target save file (alias)")

	flag.Parse()

	if from == "" || to == "" {
		fmt.Fprintln(os.Stderr, "error: --from/--source and --to/--target are required")
		flag.Usage()
		os.Exit(2)
	}

	return Config{
		From: from,
		To:   to,
	}
}

func main() {
	cfg := parse()
	fmt.Println(cfg.From, cfg.To)
}

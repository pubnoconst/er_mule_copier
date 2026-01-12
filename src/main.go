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

	return Config{From: from, To: to}
}

func printCharacterTable(source, target []*Character) {
	const (
		slotW  = 4
		nameW  = 26
		totalW = slotW + nameW*2 + 7
	)

	line := func() {
		fmt.Println("+" + repeat("-", totalW+1) + "+")
	}

	pad := func(s string, w int) string {
		if len(s) >= w {
			return s[:w]
		}
		return s + repeat(" ", w-len(s))
	}

	line()
	fmt.Printf("| %-4s | %-26s | %-26s |\n", "SLOT", "SOURCE", "TARGET")
	line()

	for i := 0; i < MaxCharacters; i++ {
		src := "(Empty)"
		tgt := "(Empty)"

		if source[i] != nil {
			src = fmt.Sprintf("Slot %d: %s", i, source[i].Name)
		}
		if target[i] != nil {
			tgt = fmt.Sprintf("Slot %d: %s", i, target[i].Name)
		}

		fmt.Printf(
			"| %4d | %-26s | %-26s |\n",
			i,
			pad(src, nameW),
			pad(tgt, nameW),
		)
	}

	line()
}

func repeat(s string, n int) string {
	out := make([]byte, 0, len(s)*n)
	for i := 0; i < n; i++ {
		out = append(out, s...)
	}
	return string(out)
}

func main() {
	cfg := parse()

	sourceData, err := os.ReadFile(cfg.From)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Unable to read source file.")
		os.Exit(1)
	}

	targetData, err := os.ReadFile(cfg.To)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Unable to read target file.")
		os.Exit(1)
	}

	sourceCharacters := ListCharacters(sourceData)
	targetCharacters := ListCharacters(targetData)

	printCharacterTable(sourceCharacters, targetCharacters)

	var sourceSlot, targetSlot int
	fmt.Println()
	fmt.Println("Enter source slot followed by target slot (e.g. \"2 5\"):")

	_, err = fmt.Fscan(os.Stdin, &sourceSlot, &targetSlot)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Invalid input.")
		os.Exit(1)
	}

	if sourceSlot < 0 || sourceSlot >= MaxCharacters {
		fmt.Fprintln(os.Stderr, "Error: Source index out of range.")
		os.Exit(1)
	}
	if targetSlot < 0 || targetSlot >= MaxCharacters {
		fmt.Fprintln(os.Stderr, "Error: Target index out of range.")
		os.Exit(1)
	}

	newSave, err := GenerateNewData(
		sourceData,
		sourceSlot,
		targetData,
		targetSlot,
	)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed generating new save (%v), exiting.\n", err)
		os.Exit(1)
	}

	backupPath, err := WriteBackup(newSave, nil)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed making backup (%v), exiting.\n", err)
		os.Exit(1)
	}

	fmt.Println("Backup written successfully as", backupPath)

	if err := WriteFile(newSave, cfg.To); err != nil {
		fmt.Fprintf(os.Stderr, "Error overwriting save file: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("File overwritten successfully -- Done!")
}

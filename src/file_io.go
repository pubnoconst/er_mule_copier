package main

import (
	"bytes"
	"crypto/md5"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

const MaxCharacters = 10

func ListActiveCharacters(data []byte) []Character {
	out := make([]Character, MaxCharacters)
	n := 0
	for i := range MaxCharacters {
		if c := NewActiveCharacter(data, i); c != nil {
			out[n] = *c
			n++
		}
	}
	return out[:n]
}

func ListCharacters(data []byte) []*Character {
	out := make([]*Character, MaxCharacters)
	for i := range MaxCharacters {
		out[i] = NewActiveCharacter(data, i)
	}
	return out
}

func ListAllCharacters(data []byte) []Character {
	out := make([]Character, MaxCharacters)
	for i := range MaxCharacters {
		out[i] = NewCharacter(data, i)
	}
	return out
}

func unixTimestamp() uint64 {
	return uint64(time.Now().Unix())
}

func WriteBackup(data []byte, destination *string) (string, error) {
	var base string
	if destination != nil {
		base = *destination
	} else {
		dir, err := os.UserHomeDir()
		if err != nil {
			return "", errors.New("unable to determine home directory")
		}
		base = filepath.Join(dir, "Documents", "er_mule_copier_backups")
		if err := os.MkdirAll(base, 0755); err != nil {
			return "", err
		}
	}

	filename := fmt.Sprintf("ER0000 backup from %d.sl2", unixTimestamp())
	fullpath := filepath.Join(base, filename)

	if err := os.WriteFile(fullpath, data, 0644); err != nil {
		return "", err
	}
	return fullpath, nil
}

func WriteFile(data []byte, filename string) error {
	backup := filename + ".bak"
	_ = os.Remove(backup) // best-effort, matches Rust behavior
	return os.WriteFile(filename, data, 0644)
}

func subslicePositions(needle, haystack []byte) []int {
	if len(needle) == 0 || len(haystack) < len(needle) {
		return nil
	}

	// worst case: every byte matches
	out := make([]int, 0, 16)
	for i := 0; i <= len(haystack)-len(needle); i++ {
		if bytes.Equal(haystack[i:i+len(needle)], needle) {
			out = append(out, i)
		}
	}
	return out
}

func GenerateNewData(
	sourceData []byte,
	sourceSlot int,
	targetData []byte,
	targetSlot int,
) ([]byte, error) {

	newsave := make([]byte, len(targetData))
	copy(newsave, targetData)

	sourceID, err := ParseSteamID(sourceData)
	if err != nil {
		return nil, err
	}
	targetID, err := ParseSteamID(targetData)
	if err != nil {
		return nil, err
	}

	sourceSlotData := ParseSaveData(sourceData, sourceSlot)
	sourceHeaderData := ParseHeaderData(sourceData, sourceSlot)

	for _, pos := range subslicePositions(sourceID[:], sourceSlotData) {
		copy(
			sourceSlotData[pos:pos+STEAM_ID_LENGTH],
			targetID[:],
		)
	}

	copy(
		newsave[GetSlotStartPosition(targetSlot):GetSlotStartPosition(targetSlot)+SLOT_LENGTH],
		sourceSlotData,
	)

	copy(
		newsave[GetHeaderStartPosition(targetSlot):GetHeaderStartPosition(targetSlot)+SAVE_HEADER_LENGTH],
		sourceHeaderData,
	)

	newsave[CHAR_ACTIVE_STATUS_START_INDEX+targetSlot] = 1

	slotHash := md5.Sum(sourceSlotData)
	copy(
		newsave[GetSlotStartPosition(targetSlot)-0x10:GetSlotStartPosition(targetSlot)],
		slotHash[:],
	)

	headerHash := md5.Sum(
		newsave[SAVE_HEADERS_SECTION_START_INDEX : SAVE_HEADERS_SECTION_START_INDEX+SAVE_HEADERS_SECTION_LENGTH],
	)

	copy(
		newsave[SAVE_HEADERS_SECTION_START_INDEX-0x10:SAVE_HEADERS_SECTION_START_INDEX],
		headerHash[:],
	)

	return newsave, nil
}

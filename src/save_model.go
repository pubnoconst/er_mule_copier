package main

import (
	"errors"
	"strconv"
	"unicode/utf16"
)

const (
	SLOT_START_INDEX                 = 0x310
	SLOT_LENGTH                      = 0x280000
	SAVE_HEADERS_SECTION_START_INDEX = 0x19003B0
	SAVE_HEADERS_SECTION_LENGTH      = 0x60000
	SAVE_HEADER_START_INDEX          = 0x1901D0E
	SAVE_HEADER_LENGTH               = 0x24C
	CHAR_ACTIVE_STATUS_START_INDEX   = 0x1901D04
	CHAR_NAME_LENGTH                 = 0x22
	STEAM_ID_LOCATION                = 0x19003B4
	STEAM_ID_LENGTH                  = 8
)

func ParseSteamID(data []byte) ([STEAM_ID_LENGTH]byte, error) {
	var out [STEAM_ID_LENGTH]byte

	if len(data) < STEAM_ID_LOCATION+STEAM_ID_LENGTH {
		return out, errors.New("data too short for steam id")
	}

	copy(out[:], data[STEAM_ID_LOCATION:STEAM_ID_LOCATION+STEAM_ID_LENGTH])
	return out, nil
}

func GetSlotStartPosition(characterSlotIndex int) int {
	return SLOT_START_INDEX +
		characterSlotIndex*0x10 +
		characterSlotIndex*SLOT_LENGTH
}

func GetHeaderStartPosition(characterSlotIndex int) int {
	return SAVE_HEADER_START_INDEX +
		characterSlotIndex*SAVE_HEADER_LENGTH
}

type Character struct {
	Index  int
	Active bool
	Name   string
}

func parseActive(data []byte, index int) bool {
	pos := CHAR_ACTIVE_STATUS_START_INDEX + index
	if pos >= len(data) {
		return false
	}
	return data[pos] == 1
}

func ParseName(data []byte, index int) string {
	start := SAVE_HEADER_START_INDEX + index*SAVE_HEADER_LENGTH
	end := start + CHAR_NAME_LENGTH

	if end > len(data) {
		return ""
	}

	u16 := make([]uint16, 0, CHAR_NAME_LENGTH/2)

	for i := start; i+1 < end; i += 2 {
		val := uint16(data[i]) | uint16(data[i+1])<<8
		if val == 0 {
			break
		}
		u16 = append(u16, val)
	}

	return string(utf16.Decode(u16))
}

func ParseSaveData(data []byte, index int) []byte {
	start := SLOT_START_INDEX + index*0x10 + index*SLOT_LENGTH
	end := start + SLOT_LENGTH

	if start >= len(data) {
		return nil
	}
	if end > len(data) {
		end = len(data)
	}

	out := make([]byte, end-start)
	copy(out, data[start:end])
	return out
}

func ParseHeaderData(data []byte, index int) []byte {
	start := SAVE_HEADER_START_INDEX + index*SAVE_HEADER_LENGTH
	end := start + SAVE_HEADER_LENGTH

	if start >= len(data) {
		return nil
	}
	if end > len(data) {
		end = len(data)
	}

	out := make([]byte, end-start)
	copy(out, data[start:end])
	return out
}

// NewCharacter generates a Character (active or inactive)
func NewCharacter(data []byte, index int) Character {
	return Character{
		Index:  index,
		Active: parseActive(data, index),
		Name:   ParseName(data, index),
	}
}

// NewActiveCharacter generates a Character only if active
func NewActiveCharacter(data []byte, index int) *Character {
	if !parseActive(data, index) {
		return nil
	}
	c := NewCharacter(data, index)
	return &c
}

func (c Character) String() string {
	return "Slot " + strconv.Itoa(c.Index) + " " + c.Name
}

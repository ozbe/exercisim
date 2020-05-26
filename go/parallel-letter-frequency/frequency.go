package letter

import "sync"

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func ConcurrentFrequency(seq []string) FreqMap {
    seqLen := len(seq)
    freqMaps := make(chan FreqMap, seqLen)
    result := FreqMap{}
    var wg sync.WaitGroup

    for _, s := range seq {
        wg.Add(1)
        go func(s string) {
            defer wg.Done()
            freqMaps <- Frequency(s)
        }(s)
    }

    wg.Wait()
    close(freqMaps)

    for f := range freqMaps {
        for k, v := range f {
            result[k] += v
        }
    }

    return result
}

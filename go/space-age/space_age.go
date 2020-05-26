package space

import (
    "fmt"
    "math"
)

type Planet string

const (
    Earth Planet = "Earth"
    Mercury Planet = "Mercury"
    Venus Planet = "Venus"
    Mars Planet = "Mars"
    Jupiter Planet = "Jupiter"
    Saturn Planet = "Saturn"
    Uranus Planet = "Uranus"
    Neptune Planet = "Neptune"
)

func Age(seconds float64, planet Planet) float64 {
    var conversionRate float64
    switch planet {
    case Earth:
        conversionRate = 1
    case Mercury:
        conversionRate = 0.2408467
    case Venus:
        conversionRate = 0.61519726
    case Mars:
        conversionRate = 1.8808158
    case Jupiter:
        conversionRate = 11.862615
    case Saturn:
        conversionRate = 29.447498
    case Uranus:
        conversionRate = 84.016846
    case Neptune:
        conversionRate = 164.79132
    default:
        panic(fmt.Sprintf("Unknown planet: %s", planet))
    }

    rawResult := seconds / 31557600 / conversionRate
    return float64(math.Round(rawResult * 100)) / 100
}

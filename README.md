# Airport Mapping

This project was initiated with the intent of comparing the performance of Rust and Python by writing code in both languages to create a GeoJSON file with quadkey added to airport data.

## Data

The original source of the data is from [Datahub.io Core Datasets](https://datahub.io/core/airport-codes), but for this project, the data was downloaded from a [Kaggle dataset](https://www.kaggle.com/datasets/jinbonnie/airport-information) made available by Kaggle user Jinbonnie.

The dataset consists of airport codes from around the world. The airport codes may refer to either the IATA airport code, a three-letter code used in passenger reservation, ticketing, and baggage-handling systems, or the ICAO airport code, which is a four-letter code used by Air Traffic Control (ATC) systems and for airports that do not have an IATA airport code, as defined by Wikipedia.

## Results

Contrary to expectations, there was almost no difference in performance between the two languages for this specific task. Below are the observed performances:

- Rust: 0.82s user, 0.08s system, 63% CPU, 1.422s total
- Python: 1.13s user, 0.09s system, 89% CPU, 1.366s total

These results indicate that this particular process may not be suitable for comparing the performance of Rust and Python as there was no significant difference in execution time. 

## Visualization

Additionally, I used Mapbox to display the created GeoJSON. 

[Map](https://masaishi.github.io/airport-mapping/)

![Map](https://github.com/masaishi/airport-mapping/blob/master/images/map.png)
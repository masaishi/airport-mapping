import csv
import json
from pyquadkey2 import quadkey as pyquadkey2

def get_airports():
	airports = {}
	with open('./data/airport_code.csv', 'r') as file:
		reader = csv.reader(file)
		next(reader)
		for row in reader:
			try:
				type_ = row[2]
				name = row[3]
				latitude_deg = float(row[4])
				longitude_deg = float(row[5])
				elevation_ft = int(row[6]) if row[6] else None
				quadkey = str(pyquadkey2.from_geo((latitude_deg, longitude_deg), 20))
				airports[row[1]] = [type_, name, latitude_deg, longitude_deg, elevation_ft, quadkey]
			except:
				print("Error: ", row)
	return airports

def write_geojson(airports):
	features = []
	for airport in airports.values():
		features.append({
			'type': 'Feature',
			'properties': {
				'type': airport[0],
				'name': airport[1],
				'elevation_ft': airport[4],
				'quadkey': airport[5]
			},
			'geometry': {
				'type': 'Point',
				'coordinates': [airport[3], airport[2]]
			}
		})
	geojson = {
		'type': 'FeatureCollection',
		'features': features
	}
	with open('./data/py_aairport_code_quadkey.geojson', 'w') as file:
		json.dump(geojson, file)

import numpy as np
import matplotlib.pyplot as plt
# Main
if __name__ == "__main__":
	airports = get_airports()
	write_geojson(airports)
	
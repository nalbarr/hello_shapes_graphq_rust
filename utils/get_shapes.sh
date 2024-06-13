#!/bin/bash

curl 'http://localhost:3000/shapes' \
-X POST \
-H 'content-type: application/json' \
--data '{
	"query": "{ getShapes { id name } }"
	}'

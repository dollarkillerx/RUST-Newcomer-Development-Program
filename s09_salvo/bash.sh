#!/bin/sh
image_name=ru_system
version=1.0.0

docker rmi -f github.tailbd724f.ts.net/$image_name:$version
docker build -f Dockerfile -t github.tailbd724f.ts.net/$image_name:$version  .

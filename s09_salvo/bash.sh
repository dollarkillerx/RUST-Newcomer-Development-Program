#!/bin/sh
image_name=ru_system
version=1.0.1

docker rmi -f github.tailbd724f.ts.net:5000/$image_name:$version
docker build -f Dockerfile -t github.tailbd724f.ts.net:5000/$image_name:$version  .
docker push github.tailbd724f.ts.net:5000/$image_name:$version

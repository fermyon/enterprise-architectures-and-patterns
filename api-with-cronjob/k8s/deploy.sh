#! /bin/bash

set -euo pipefail

current_directory=$(pwd)

REGISTRY=ttl.sh
TAG=12h

APP_ARTIFACT=$REGISTRY/spin-api:$TAG
CRON_ARTIFACT=$REGISTRY/spin-cron:$TAG

SCHEDULE="*/2 * * * *" # Every 2nd minute

# Deploy Redis
helm upgrade --install redis oci://registry-1.docker.io/bitnamicharts/redis -n redis --create-namespace

# Grab the redis secret
redis_password=$(kubectl get secret --namespace redis redis -o jsonpath="{.data.redis-password}" | base64 -d)

# Create a secret for the Spin App and the Cron Job
sed "s|PASSWORD|${redis_password}|g" ./k8s.rtc.tmpl > ./runtime-config.toml
kubectl delete secret rtc --ignore-not-found true
kubectl create secret generic rtc --from-file=./runtime-config.toml

# Build & Push the Spin App and the Spin Cron App

cd ..
spin registry push --build -f spin.toml $APP_ARTIFACT
spin registry push --build -f spin-cron.toml $CRON_ARTIFACT

cd $current_directory
# Deploy the Spin App
sed "s|ARTIFACT|${APP_ARTIFACT}|g" ./spin-app.tmpl > ./spin-app.yaml
kubectl apply -f ./spin-app.yaml

# Deploy the Spin Cron Trigger
sed -e "s|ARTIFACT|${CRON_ARTIFACT}|g" -e "s|SCHEDULE|${SCHEDULE}|g" ./spin-cron.tmpl > ./spin-cron.yaml
kubectl apply -f ./spin-cron.yaml

# Delete generated files again
rm spin-cron.yaml
rm spin-app.yaml
rm runtime-config.toml

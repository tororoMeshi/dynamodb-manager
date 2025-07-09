#!/bin/bash
set -e

# .env 読み込み
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
else
  echo "❌ .env ファイルがありません。"
  exit 1
fi

docker run --rm -it \
  -v "$PWD/terraform":/workspace \
  -w /workspace \
  -e AWS_ACCESS_KEY_ID \
  -e AWS_SECRET_ACCESS_KEY \
  -e AWS_DEFAULT_REGION \
  hashicorp/terraform:latest "$@"

# # Terraform
# ./tf.sh init
# ./tf.sh plan
# ./tf.sh apply

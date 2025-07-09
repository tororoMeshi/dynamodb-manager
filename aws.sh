#!/bin/bash
# aws.sh
set -e

if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
else
  echo "❌ .env ファイルがありません。"
  exit 1
fi

# echo "🔐 Verifying AWS credentials..."
# docker run --rm -i \
#   -e AWS_ACCESS_KEY_ID \
#   -e AWS_SECRET_ACCESS_KEY \
#   -e AWS_DEFAULT_REGION \
#   amazon/aws-cli sts get-caller-identity

# 実際のコマンドを実行
echo "🚀 Running: aws $@"
docker run --rm -i \
  -e AWS_ACCESS_KEY_ID \
  -e AWS_SECRET_ACCESS_KEY \
  -e AWS_DEFAULT_REGION \
  amazon/aws-cli "$@"

# # AWS CLI
# ./aws.sh sts get-caller-identity

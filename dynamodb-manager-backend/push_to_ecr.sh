#!/usr/bin/env bash
set -euo pipefail

AWS_REGION="${AWS_REGION:-ap-northeast-1}"
AWS_ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
PROJECT_NAME="${PROJECT_NAME:-$(basename $(pwd))}"
IMAGE_TAG=$(date +%Y%m%d%H%M)
if [ $# -ge 1 ]; then IMAGE_TAG="$1"; fi

REPO_URI="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${PROJECT_NAME}"

echo "==> Logging in to Amazon ECR..."
aws ecr get-login-password --region "$AWS_REGION" | \
  docker login --username AWS --password-stdin "$REPO_URI"

echo "==> Checking ECR repository exists (Terraform should manage it)..."
if ! aws ecr describe-repositories --repository-names "${PROJECT_NAME}" --region "$AWS_REGION" >/dev/null 2>&1; then
  echo "❌ ECR repository ${PROJECT_NAME} does not exist. Please create it via Terraform."
  exit 1
fi

echo "==> Building and pushing image to ECR: ${REPO_URI}:${IMAGE_TAG}"
docker buildx create --use --name multiarch-ecr-builder >/dev/null 2>&1 || true

docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag "${REPO_URI}:${IMAGE_TAG}" \
  --push \
  .

echo "✅ Multi-arch image pushed to: ${REPO_URI}:${IMAGE_TAG}"

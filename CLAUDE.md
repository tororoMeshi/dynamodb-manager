# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## 📦 Project Overview

DynamoDB Manager is a lightweight web-based tool for viewing and annotating the structure of DynamoDB tables. It allows users to browse tables and attributes, leave memos on usage, and inspect example data.

---

## 🛠️ Development Workflow

### Infrastructure Setup

1. Initialize Terraform:
   ```bash
   ./tf.sh init
```

2. Review infrastructure changes:

   ```bash
   ./tf.sh plan
   ```

3. Apply infrastructure:

   ```bash
   ./tf.sh apply
   ```

4. Destroy infrastructure (if needed):

   ```bash
   ./tf.sh destroy
   ```

### Rust Lambda Development

* Generate a new Rust Lambda project with:

  ```bash
  ./new-rust.sh <project-name>
  ```

* Build and push multi-arch Docker image to ECR:

  ```bash
  ./push_to_ecr.sh
  ```

* Lint and security check:

  ```bash
  ./lint.sh
  ```

* Update Lambda function to use new ECR image:

  ```bash
  ./aws.sh lambda update-function-code \
    --function-name dynamodb-manager-backend \
    --image-uri <ECR_IMAGE_URI>
  ```

> Tip: Get the image URI with:
>
> ```bash
> ./aws.sh ecr describe-images \
>   --repository-name dynamodb-admin-backend
> ```

### Frontend Deployment (S3 + CloudFront)

* Sync HTML/JS/CSS to S3:

  ```bash
  ./aws.sh s3 sync ./frontend/ s3://dynamodb-admin-ui-${ACCOUNT_ID}/
  ```

* Invalidate CloudFront cache after upload:

  ```bash
  ./aws.sh cloudfront create-invalidation \
    --distribution-id <DISTRIBUTION_ID> \
    --paths "/*"
  ```

---

## 🧾 Terraform Directory Structure

```text
terraform/
├── main.tf          # Provider and backend configuration
├── variables.tf     # AWS region, environment, and account ID
├── locals.tf        # Shared tags and project constants
├── dynamodb.tf      # Metadata table definition (memos per attribute)
├── ecr.tf           # ECR repo with lifecycle policies
├── lambda.tf        # Lambda function linked to ECR image
├── s3.tf            # Frontend S3 bucket definition
└── cloudfront.tf    # CloudFront distribution for static site
```

---

## 🧱 Infrastructure Components

| Component       | Technology           | Purpose                                |
| --------------- | -------------------- | -------------------------------------- |
| Backend         | Rust + `lambda_http` | API for listing tables, updating memos |
| Frontend        | HTML/JS/CSS          | Deployed to S3/CloudFront              |
| Storage         | DynamoDB             | Application data and metadata          |
| Container Image | Docker + ECR         | Lambda container image                 |
| Hosting         | S3 + CloudFront      | Static site delivery + HTTPS + CDN     |

---

## 🔌 API Overview

**Example Endpoints:**

```http
GET /tables
GET /tables/{table}
GET /tables/{table}/sample
PUT /tables/{table}/{attribute}
```

**Sample curl:**

```bash
curl https://<cloudfront-domain>/tables

curl -X PUT https://<cloudfront-domain>/tables/users/email \
  -H "Content-Type: application/json" \
  -d '{"memo": "ユーザーのメールアドレスです"}'
```

---

## 🌍 CloudFront Configuration Notes

* CloudFront uses **OAC** (Origin Access Control) to access S3
* Set `index.html` and `error.html` in S3 for static hosting
* Invalidate cache on redeploy (`/*`)

**CORS Handling:**
Lambda responses must include:

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

**CloudFront:**

* Disable caching for `OPTIONS` method and `Origin` header

---

## 🔐 IAM & Security

### Lambda Execution Role Permissions

```json
[
  "dynamodb:ListTables",
  "dynamodb:GetItem",
  "dynamodb:PutItem",
  "dynamodb:Scan",
  "logs:CreateLogGroup",
  "logs:CreateLogStream",
  "logs:PutLogEvents",
  "ecr:GetAuthorizationToken",
  "ecr:BatchGetImage",
  "ecr:GetDownloadUrlForLayer"
]
```

* IAM roles are attached automatically via Terraform
* Least-privilege policy model enforced

---

## 📋 Environment Setup

1. Copy and configure `.env` from `.env.example`
2. All commands (Terraform, AWS CLI, Docker) run via helper scripts:

   * `./aws.sh` – AWS CLI via Docker
   * `./tf.sh` – Terraform via Docker
   * `./new-rust.sh` – Rust project + container build helpers

---

## 🧪 Testing Tips

* Backend Lambda function can be tested via Function URL or API Gateway
* Use `curl` to verify endpoints
* Use `aws logs tail` to inspect CloudWatch logs if needed

---

## 🔮 Future Enhancements

* Memo version history and change tracking
* Attribute status: Active / Deprecated
* Grouping and filtering for metadata
* Markdown schema auto-generation
* Read frequency statistics from source tables

---

This project aims to bring clarity to growing DynamoDB schemas through collaborative annotation and visualization. Fast, serverless, and infrastructure-as-code from the start.

```

---

日本語で回答をお願いします。
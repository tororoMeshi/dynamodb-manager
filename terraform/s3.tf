resource "aws_s3_bucket" "frontend" {
  bucket = "dynamodb-admin-ui-${var.account_id}"

  tags = {
    Name = "DynamoDB Admin UI"
  }
}

resource "aws_s3_bucket_website_configuration" "frontend" {
  bucket = aws_s3_bucket.frontend.bucket

  index_document {
    suffix = "index.html"
  }

  error_document {
    key = "index.html"
  }
}

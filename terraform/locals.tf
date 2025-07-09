locals {
  common_tags = {
    environment = "prod"
    project     = "dynamodb-admin"
    owner       = "your-name"
    purpose     = "lambda-backend"
    managed_by  = "terraform"
  }
}

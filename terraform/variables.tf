variable "region" {
  description = "AWS region"
  type        = string
}

variable "account_id" {
  description = "AWS account ID"
  type        = string
}

variable "environment" {
  type        = string
  description = "Environment name"
  default     = "prod"
}

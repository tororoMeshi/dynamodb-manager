resource "aws_dynamodb_table" "notes" {
  name           = "Notes"
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "id"

  attribute {
    name = "id"
    type = "S"
  }

  tags = {
    Name = "Notes Table"
  }
}

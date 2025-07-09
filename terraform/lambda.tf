resource "aws_lambda_function" "dynamodb_manager_backend" {
  function_name = "dynamodb-manager-backend"
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.dynamodb_admin_backend.repository_url}:latest"

  role = aws_iam_role.lambda_exec_role.arn

  memory_size      = 128
  timeout          = 10
  publish          = true
  architectures    = ["x86_64"]
  
  environment {
    variables = {
      DYNAMODB_TABLE = aws_dynamodb_table.notes.name
      RUST_LOG       = "info"
    }
  }
}

resource "aws_iam_role" "lambda_exec_role" {
  name = "lambda_exec_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Principal = {
        Service = "lambda.amazonaws.com"
      }
      Effect = "Allow"
      Sid = ""
    }]
  })
}

resource "aws_iam_role_policy" "lambda_exec_policy" {
  name   = "lambda_exec_policy"
  role   = aws_iam_role.lambda_exec_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:*:*:*"
      },
      {
        Effect = "Allow"
        Action = [
          "dynamodb:ListTables",
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:Scan",
          "dynamodb:UpdateItem"
        ]
        Resource = aws_dynamodb_table.notes.arn
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:GetAuthorizationToken",
          "ecr:BatchGetImage",
          "ecr:GetDownloadUrlForLayer"
        ]
        Resource = "*"
      }
    ]
  })
}

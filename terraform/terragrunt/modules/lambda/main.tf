resource "aws_lambda_function" "terraform_function" {
    function_name = "${var.env}-terraform-function"
    role = aws_iam_role.terraform_function_role.arn
    runtime = "nodejs12.x"

    filename = "./package.zip"
    handler =  "handler.main"

     depends_on = [
        aws_iam_role_policy_attachment.terraform_function_policy_attachment,
    ]
}

resource "aws_iam_role" "terraform_function_role" {
    name = "${var.env}-terraform-function-role"
    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "terraform_function_policy_attachment" {
    role = aws_iam_role.terraform_function_role.name
    policy_arn = aws_iam_policy.terraform_function_policy.arn
}

resource "aws_iam_policy" "terraform_function_policy" {
    name = "${var.env}-terraform-function-policy"
    description = "Policy just for AWS lambda function function"

    policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": "logs:CreateLogGroup",
            "Resource": "arn:aws:logs:*"
        },
        {
            "Effect": "Allow",
            "Action": [
                "logs:CreateLogStream",
                "logs:PutLogEvents"
            ],
            "Resource": [
                "arn:aws:logs:eu-central-1:125906467106:log-group:/aws/lambda/terraform-function:*"
            ]
        }
    ]
}
    EOF
}